use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use posty::{
    RequestData,
    executor::{ExecutionMode, Executor, RequestSettings},
};
use std::{sync::Arc, time::Duration};
use tokio::runtime::Runtime;

// ── shared helpers ─────────────────────────────────────────────────────────────

fn rt() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn make_get(url: &str) -> RequestData {
    RequestData {
        method: "GET".to_string(),
        url: url.to_string(),
        ..Default::default()
    }
}

/// Spins up a local `mockito` server that responds 200 immediately.
/// Returns the base URL and the `mockito::ServerGuard` — drop the guard to
/// shut the server down.
async fn local_server() -> (String, mockito::ServerGuard) {
    let mut server = mockito::Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(200)
        .with_body("ok")
        .expect_at_least(1)
        .create_async()
        .await;
    let url = server.url();
    (url, server)
}

// 1.  Worker-count sweep  (fixed 200 requests, vary workers)
//     Answers: "how many workers give peak RPS against a local server?"
fn bench_worker_sweep(c: &mut Criterion) {
    let rt = rt();
    let (url, _guard) = rt.block_on(local_server());

    let mut group = c.benchmark_group("worker_sweep");
    group.throughput(Throughput::Elements(200));
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    for workers in [1, 2, 4, 8, 16, 32, 64] {
        group.bench_with_input(BenchmarkId::from_parameter(workers), &workers, |b, &w| {
            let settings = RequestSettings {
                worker_count: Some(w),
                request_count: 200,
                ..Default::default()
            };
            let exec = Arc::new(Executor::new(settings, ExecutionMode::MaxThroughput));
            let req = make_get(&url);

            b.to_async(&rt).iter(|| {
                let exec = Arc::clone(&exec);
                let req = req.clone();
                async move { exec.run(req, None).await.unwrap() }
            });
        });
    }
    group.finish();
}

// 2.  Request-count scaling  (fixed 8 workers, vary total requests)
//     Answers: "does total time scale linearly with request count?"
fn bench_request_scaling(c: &mut Criterion) {
    let rt = rt();
    let (url, _guard) = rt.block_on(local_server());

    let mut group = c.benchmark_group("request_scaling");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    for &count in &[10u64, 50, 100, 500, 1_000] {
        group.throughput(Throughput::Elements(count));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &n| {
            let settings = RequestSettings {
                worker_count: Some(8),
                request_count: n as usize,
                ..Default::default()
            };
            let exec = Arc::new(Executor::new(settings, ExecutionMode::MaxThroughput));
            let req = make_get(&url);

            b.to_async(&rt).iter(|| {
                let exec = Arc::clone(&exec);
                let req = req.clone();
                async move { exec.run(req, None).await.unwrap() }
            });
        });
    }
    group.finish();
}

// 3.  Execution-mode overhead  (fixed 100 requests / 8 workers)
//     Answers: "how much does body extraction cost vs. fire-and-forget?"
fn bench_execution_modes(c: &mut Criterion) {
    let rt = rt();
    let (url, _guard) = rt.block_on(local_server());

    let mut group = c.benchmark_group("execution_modes");
    group.throughput(Throughput::Elements(100));
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    let modes = [
        ("MaxThroughput", ExecutionMode::MaxThroughput),
        ("Balanced", ExecutionMode::Balanced),
        ("FullTracking", ExecutionMode::FullTracking),
    ];

    for (label, mode) in modes {
        let settings = RequestSettings {
            worker_count: Some(8),
            request_count: 100,
            ..Default::default()
        };
        let exec = Arc::new(Executor::new(settings, mode));
        let req = make_get(&url);

        group.bench_function(label, |b| {
            b.to_async(&rt).iter(|| {
                let exec = Arc::clone(&exec);
                let req = req.clone();
                async move {
                    // FullTracking/Balanced need a live receiver or the send
                    // will fail once the implicit channel fills.
                    let (tx, mut rx) = tokio::sync::mpsc::channel(128);
                    let handle = tokio::spawn(async move { exec.run(req, Some(tx)).await });
                    // Drain so the channel never blocks workers
                    while rx.recv().await.is_some() {}
                    handle.await.unwrap().unwrap()
                }
            });
        });
    }
    group.finish();
}

fn bench_unbounded_concurrency(c: &mut Criterion) {
    let rt = rt();
    let (url, _guard) = rt.block_on(local_server());

    let mut group = c.benchmark_group("unbounded_concurrency");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    for &count in &[50u64, 200, 500] {
        group.throughput(Throughput::Elements(count));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &n| {
            let settings = RequestSettings {
                worker_count: None, // no semaphore — all tasks fire at once
                request_count: n as usize,
                ..Default::default()
            };
            let exec = Arc::new(Executor::new(settings, ExecutionMode::MaxThroughput));
            let req = make_get(&url);

            b.to_async(&rt).iter(|| {
                let exec = Arc::clone(&exec);
                let req = req.clone();
                async move { exec.run(req, None).await.unwrap() }
            });
        });
    }
    group.finish();
}

// ══════════════════════════════════════════════════════════════════════════════
// 5.  Channel back-pressure  (slow consumer vs. fast executor)
//     Answers: "does a slow consumer stall the workers?"
// ══════════════════════════════════════════════════════════════════════════════
fn bench_channel_backpressure(c: &mut Criterion) {
    let rt = rt();
    let (url, _guard) = rt.block_on(local_server());

    let mut group = c.benchmark_group("channel_backpressure");
    group.throughput(Throughput::Elements(50));
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    for &cap in &[1usize, 8, 64] {
        let settings = RequestSettings {
            worker_count: Some(8),
            request_count: 50,
            channel_capacity: Some(cap),
            ..Default::default()
        };
        let exec = Arc::new(Executor::new(settings, ExecutionMode::FullTracking));
        let req = make_get(&url);

        group.bench_with_input(BenchmarkId::new("capacity", cap), &cap, |b, _| {
            b.to_async(&rt).iter(|| {
                let exec = Arc::clone(&exec);
                let req = req.clone();
                async move {
                    let (tx, mut rx) = tokio::sync::mpsc::channel(cap);
                    let handle = tokio::spawn(async move { exec.run(req, Some(tx)).await });
                    while rx.recv().await.is_some() {}
                    handle.await.unwrap().unwrap()
                }
            });
        });
    }
    group.finish();
}
fn bench_rps_vs_count(c: &mut Criterion) {
    let rt = rt();
    let (url, _guard) = rt.block_on(local_server());

    let mut group = c.benchmark_group("rps_vs_count");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(30);

    for &count in &[100u64, 500, 1_000, 5_000, 10_000, 100_000, 1_000_000] {
        group.throughput(Throughput::Elements(count));
        group.bench_with_input(BenchmarkId::new("requests", count), &count, |b, &n| {
            let settings = RequestSettings {
                worker_count: None, // unbounded — no semaphore
                request_count: n as usize,
                ..Default::default()
            };
            let exec = Arc::new(Executor::new(settings, ExecutionMode::MaxThroughput));
            let req = make_get(&url);

            b.to_async(&rt).iter(|| {
                let exec = Arc::clone(&exec);
                let req = req.clone();
                async move { exec.run(req, None).await.unwrap() }
            });
        });
    }
    group.finish();
}
criterion_group!(
    benches,
    bench_worker_sweep,
    bench_request_scaling,
    bench_execution_modes,
    bench_unbounded_concurrency,
    bench_channel_backpressure,
    bench_rps_vs_count,
);
criterion_main!(benches);
