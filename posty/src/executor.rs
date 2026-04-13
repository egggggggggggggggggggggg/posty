use crate::{IntoRequestError, IntoResponseError, RequestData, ResponseData, collection::FileData};
use reqwest::Client;
use std::{
    sync::Arc,
    time::{Duration, Instant},
    usize,
};
use tokio::sync::{Semaphore, mpsc};

#[derive(Debug)]
pub enum ExecutorError {
    IntoRequest(IntoRequestError),
    Worker(reqwest::Error),
    ResponseParse(IntoResponseError),
    ChannelClosed,
}
impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResponseParse(a) => write!(f, "failed to build response: {a:?}"),
            Self::IntoRequest(e) => write!(f, "failed to build request: {e:?}"),
            Self::Worker(e) => write!(f, "worker error: {e}"),
            Self::ChannelClosed => write!(f, "response channel closed unexpectedly"),
        }
    }
}

impl std::error::Error for ExecutorError {}

#[derive(Default, Clone, Copy)]
///What should the executor do with the response of the rquest.
///Balanced - Tracks status codes and minor info like status, headers, etc.
///MaxThroughput - Discards the whole response. Only cares about amount of requests executed.
///FullTracking - Tracks all of the info, like body, etc, Default as the user oftentimes only sends one
///request which they want to be able to fully examine.
pub enum ExecutionMode {
    Balanced,
    MaxThroughput,
    #[default]
    FullTracking,
}

#[derive(Debug, Clone)]
pub struct RequestSettings {
    /// Number of worker tasks running concurrently.
    /// Each worker holds a permit from a [`Semaphore`] while executing.
    /// Increasing this raises throughput up to the point where the remote
    /// server or the network becomes the bottleneck.
    pub worker_count: Option<usize>,

    /// Total number of requests to dispatch.
    ///
    /// The work is distributed evenly across all workers.  Set to `0` to run
    /// indefinitely until the caller drops the `Executor` or cancels the task.
    pub request_count: usize,

    /// Per-request timeout.  `None` means no timeout.
    pub timeout: Option<Duration>,

    /// Capacity of the internal work-distribution channel.
    ///
    /// Larger values smooth out bursty workers at the cost of more memory.
    /// Defaults to `worker_count * 4` if `None`.
    pub channel_capacity: Option<usize>,
}

impl Default for RequestSettings {
    fn default() -> Self {
        Self {
            worker_count: Some(4),
            request_count: 1,
            timeout: None,
            channel_capacity: None,
        }
    }
}

impl RequestSettings {
    pub fn new(worker_count: usize, request_count: usize) -> Self {
        Self {
            worker_count: Some(worker_count),
            request_count,
            ..Default::default()
        }
    }

    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.timeout = Some(t);
        self
    }

    pub fn with_channel_capacity(mut self, cap: usize) -> Self {
        self.channel_capacity = Some(cap);
        self
    }
}
///IO is excluded from this. If the frontend wants some enum to hold both UI related stuff and
///application logic, they can define their own enum for that and use that.
pub enum AppEvent {
    Tick,
    Response(ResponseData<'static>),
    InvalidRequest(IntoRequestError),
    FailedExecution(reqwest::Error),
    ChangeDisplay(FileData),
}
pub enum RequestError {
    IntoError(IntoRequestError),
}
pub struct Executor {
    settings: RequestSettings,
    mode: ExecutionMode,
    client: Client,
}
impl Executor {
    /// Build an `Executor`.  A single [`reqwest::Client`] is shared across all
    /// workers so that the connection pool is reused.
    pub fn new(settings: RequestSettings, mode: ExecutionMode) -> Self {
        let mut builder = Client::builder();
        if let Some(t) = settings.timeout {
            builder = builder.timeout(t);
        }
        let client = builder.build().expect("failed to build reqwest client");
        Self {
            settings,
            mode,
            client,
        }
    }

    /// Build an `Executor` with a pre-configured [`reqwest::Client`].
    ///
    /// Useful when you need custom TLS roots, proxy settings, etc.
    pub fn with_client(settings: RequestSettings, mode: ExecutionMode, client: Client) -> Self {
        Self {
            settings,
            mode,
            client,
        }
    }
    /// Execute `settings.request_count` copies of `request`.
    ///
    /// Workers run concurrently, bounded by `settings.worker_count`.
    /// Each response is forwarded on `response_tx` according to the configured
    /// [`ExecutionMode`].  Pass `None` when you only care about side-effects or
    /// are using `MaxThroughput` mode.
    pub async fn run(
        &self,
        request: RequestData,
        response_tx: Option<mpsc::Sender<ResponseData<'static>>>,
    ) -> Result<ExecutorStats, ExecutorError> {
        let request = Arc::new(request);
        let mode = self.mode;
        // Change this to a more reasonable limit. On my PC, theres basically no difference between
        // 100 and 256 so maybe limit to around the hundreds. The addition of unbounded is basically
        // useless as it ends up overwhleming the request sending leading to a 99% failure rate.
        let concurrency = self.settings.worker_count.unwrap_or(Semaphore::MAX_PERMITS);
        let semaphore = Arc::new(Semaphore::new(concurrency));
        let response_tx = response_tx.map(Arc::new);

        let request_count = self.settings.request_count;
        let mut handles = Vec::with_capacity(request_count);

        for _ in 0..request_count {
            let sem = Arc::clone(&semaphore);
            let client = self.client.clone();
            let req = Arc::clone(&request);
            let tx = response_tx.clone();

            handles.push(tokio::spawn(async move {
                // Each task acquires a permit before executing —
                // this is now the *only* thing controlling concurrency.
                let _permit = sem.acquire().await.expect("semaphore closed");
                execute_one(client, req, mode, tx).await
            }));
        }

        let mut stats = ExecutorStats::default();
        for handle in handles {
            match handle.await {
                Ok(Ok(s)) => stats.merge(s),
                Ok(Err(ExecutorError::Worker(_))) => stats.failures += 1, // count, don't abort
                Ok(Err(e)) => return Err(e),
                Err(_) => {}
            }
        }

        Ok(stats)
    }
    /// Fire-and-forget: returns immediately after spawning all workers.
    /// Responses are streamed to `response_tx`.
    pub fn spawn(
        self: Arc<Self>,
        request: RequestData,
        response_tx: Option<mpsc::Sender<ResponseData<'static>>>,
    ) -> tokio::task::JoinHandle<Result<ExecutorStats, ExecutorError>> {
        tokio::spawn(async move { self.run(request, response_tx).await })
    }
}
async fn execute_one(
    client: Client,
    req_data: Arc<RequestData>,
    mode: ExecutionMode,
    tx: Option<Arc<mpsc::Sender<ResponseData<'static>>>>,
) -> Result<WorkerStats, ExecutorError> {
    let req = req_data
        .as_ref()
        .clone()
        .into_request(&client)
        .map_err(ExecutorError::IntoRequest)?;

    let start = Instant::now();
    let response = client.execute(req).await.map_err(ExecutorError::Worker)?;
    let latency = start.elapsed();

    match mode {
        ExecutionMode::MaxThroughput => {}
        ExecutionMode::Balanced | ExecutionMode::FullTracking => {
            if let Some(ref tx) = tx {
                let data = ResponseData::extract_with_body(latency, response)
                    .await
                    .map_err(ExecutorError::ResponseParse)?;
                if tx.send(data).await.is_err() {
                    return Err(ExecutorError::ChannelClosed);
                }
            }
        }
    }

    Ok(WorkerStats {
        successes: 1,
        failures: 0,
    })
}
/// Aggregated execution statistics returned by [`Executor::run`].
#[derive(Debug, Default)]
pub struct ExecutorStats {
    pub total_requests: usize,
    pub successes: usize,
    pub failures: usize,
}

impl ExecutorStats {
    fn merge(&mut self, w: WorkerStats) {
        self.successes += w.successes;
        self.failures += w.failures;
        self.total_requests += w.successes + w.failures;
    }
}
/// Per-worker counters merged into [`ExecutorStats`] at the end of a run.
#[derive(Debug, Default)]
struct WorkerStats {
    successes: usize,
    failures: usize,
}
