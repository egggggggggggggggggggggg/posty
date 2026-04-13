// This crate defines the core application logic and must remain independent of any
// presentation layer (GUI, TUI, etc.).
//
// It should not depend on rendering libraries (e.g. crossterm, ratatui, iced),
// nor contain UI-specific logic such as formatting, styling, or layout concerns.
//
// The goal is to keep this crate platform-agnostic, so different frontends can
// consume it without requiring workarounds or refactoring.

use std::time::Instant;

use posty::{
    RequestData,
    executor::{Executor, RequestSettings},
};

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
///Minor performance bench code,
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instant = Instant::now();
    let (url, _guard) = local_server().await;
    let req = RequestData {
        method: "GET".to_string(),
        url: url,
        ..Default::default()
    };
    let settings = RequestSettings {
        worker_count: Some(256),
        request_count: 100_000_000,
        ..Default::default()
    };
    let exec = Executor::new(settings, posty::executor::ExecutionMode::MaxThroughput);
    let stats = exec.run(req, None).await.unwrap();
    let elapsed = instant.elapsed();
    println!("Took {} seconds to run", elapsed.as_secs_f64());
    println!("Stats: {:?}", stats);
    Ok(())
}
