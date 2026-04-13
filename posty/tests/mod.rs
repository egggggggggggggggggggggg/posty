use mockito::Server;
use posty::{
    Auth, KVPair, RequestBody, RequestData,
    executor::{ExecutionMode, Executor, ExecutorError, RequestSettings},
};
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;

fn make_request(url: &str) -> RequestData {
    RequestData {
        method: "GET".to_string(),
        url: url.to_string(),
        ..Default::default()
    }
}

fn default_executor() -> Executor {
    Executor::new(RequestSettings::default(), ExecutionMode::FullTracking)
}

// Single-request smoke tests

#[tokio::test]
async fn single_request_returns_ok_stats() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    let exec = default_executor();
    let req = make_request(&server.url());

    let stats = exec.run(req, None).await.unwrap();

    mock.assert_async().await;
    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.successes, 1);
    assert_eq!(stats.failures, 0);
}

#[tokio::test]
async fn response_body_is_forwarded_to_channel() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(200)
        .with_body(r#"{"hello":"world"}"#)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let exec = default_executor();
    let req = make_request(&server.url());

    let (tx, mut rx) = mpsc::channel(4);
    exec.run(req, Some(tx)).await.unwrap();

    let resp = rx.recv().await.expect("expected a response");
    assert_eq!(resp.status.as_u16(), 200);
    let body = resp.body.unwrap();
    assert!(body.contains("hello"));
}

// Multi-request / concurrency
#[tokio::test]
async fn multiple_requests_all_dispatched() {
    let mut server = Server::new_async().await;
    // Allow any number of hits
    server
        .mock("GET", "/")
        .with_status(200)
        .expect_at_least(5)
        .create_async()
        .await;

    let settings = RequestSettings {
        worker_count: Some(2),
        request_count: 5,
        ..Default::default()
    };
    let exec = Executor::new(settings, ExecutionMode::FullTracking);
    let req = make_request(&server.url());

    let (tx, mut rx) = mpsc::channel(10);
    let stats = exec.run(req, Some(tx)).await.unwrap();

    assert_eq!(stats.total_requests, 5);
    assert_eq!(stats.successes, 5);

    // Drain channel and verify all 5 responses arrived
    let mut count = 0;
    while let Ok(r) = rx.try_recv() {
        assert_eq!(r.status.as_u16(), 200);
        count += 1;
    }
    assert_eq!(count, 5);
}

// ExecutionMode variants
#[tokio::test]
async fn max_throughput_does_not_send_to_channel() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    let exec = Executor::new(RequestSettings::default(), ExecutionMode::MaxThroughput);
    let req = make_request(&server.url());

    let (tx, mut rx) = mpsc::channel(4);
    let stats = exec.run(req, Some(tx)).await.unwrap();

    assert_eq!(stats.successes, 1);
    // Nothing should have been sent
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn balanced_mode_sends_response_to_channel() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(201)
        .create_async()
        .await;

    let exec = Executor::new(RequestSettings::default(), ExecutionMode::Balanced);
    let req = make_request(&server.url());

    let (tx, mut rx) = mpsc::channel(4);
    exec.run(req, Some(tx)).await.unwrap();

    let resp = rx.recv().await.unwrap();
    assert_eq!(resp.status.as_u16(), 201);
}
// Request features: params, headers, auth, body
#[tokio::test]
async fn query_params_are_appended() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/search")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "q".into(),
            "rust".into(),
        )]))
        .with_status(200)
        .create_async()
        .await;

    let mut req = make_request(&format!("{}/search", server.url()));
    req.params = vec![KVPair {
        key: "q".into(),
        value: "rust".into(),
        enabled: true,
        sensitive: false,
    }];

    let stats = default_executor().run(req, None).await.unwrap();
    assert_eq!(stats.successes, 1);
}

#[tokio::test]
async fn disabled_params_are_not_sent() {
    let mut server = Server::new_async().await;
    // The mock will only match requests WITHOUT the "secret" param
    server
        .mock("GET", "/")
        .match_query(mockito::Matcher::Missing)
        .with_status(200)
        .create_async()
        .await;

    let mut req = make_request(&server.url());
    req.params = vec![KVPair {
        key: "secret".into(),
        value: "42".into(),
        enabled: false, // disabled → must be absent
        sensitive: true,
    }];

    let stats = default_executor().run(req, None).await.unwrap();
    assert_eq!(stats.successes, 1);
}
#[tokio::test]
async fn custom_headers_are_forwarded() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .match_header("x-custom", "test-value")
        .with_status(200)
        .create_async()
        .await;

    let mut req = make_request(&server.url());
    req.headers = vec![KVPair {
        key: "x-custom".into(),
        value: "test-value".into(),
        enabled: true,
        sensitive: false,
    }];

    let stats = default_executor().run(req, None).await.unwrap();
    assert_eq!(stats.successes, 1);
}

#[tokio::test]
async fn bearer_auth_sets_authorization_header() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .match_header("authorization", "Bearer my-token")
        .with_status(200)
        .create_async()
        .await;

    let mut req = make_request(&server.url());
    req.auth = Some(Auth::Bearer {
        token: "my-token".into(),
    });

    let stats = default_executor().run(req, None).await.unwrap();
    assert_eq!(stats.successes, 1);
}

#[tokio::test]
async fn basic_auth_sets_authorization_header() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .match_header("authorization", mockito::Matcher::Regex("^Basic ".into()))
        .with_status(200)
        .create_async()
        .await;

    let mut req = make_request(&server.url());
    req.auth = Some(Auth::Basic {
        username: "user".into(),
        password: "pass".into(),
    });

    let stats = default_executor().run(req, None).await.unwrap();
    assert_eq!(stats.successes, 1);
}

#[tokio::test]
async fn json_body_sets_content_type() {
    let mut server = Server::new_async().await;
    server
        .mock("POST", "/")
        .match_header("content-type", "application/json")
        .match_body(r#"{"key":"value"}"#)
        .with_status(200)
        .create_async()
        .await;

    let mut req = make_request(&server.url());
    req.method = "POST".into();
    req.body = Some(RequestBody::Json(r#"{"key":"value"}"#.into()));

    let stats = default_executor().run(req, None).await.unwrap();
    assert_eq!(stats.successes, 1);
}

// Error / edge cases
#[tokio::test]
async fn invalid_method_returns_error() {
    let req = RequestData {
        method: "NOT A METHOD!!!".into(),
        url: "http://localhost".into(),
        ..Default::default()
    };

    let result = default_executor().run(req, None).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ExecutorError::IntoRequest(_)));
}

#[tokio::test]
async fn timeout_is_respected() {
    let mut server = Server::new_async().await;
    // Respond slowly (2 s) — our timeout is 50 ms
    server
        .mock("GET", "/")
        .with_status(200)
        .with_chunked_body(|_| {
            std::thread::sleep(Duration::from_secs(1));
            Ok(())
        })
        .create_async()
        .await;

    let settings = RequestSettings::default().with_timeout(Duration::from_millis(50));
    let exec = Executor::new(settings, ExecutionMode::FullTracking);

    let result = exec.run(make_request(&server.url()), None).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ExecutorError::Worker(_)));
}

#[tokio::test]
async fn spawn_returns_stats_via_join_handle() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    let exec = Arc::new(default_executor());
    let req = make_request(&server.url());

    let handle = exec.spawn(req, None);
    let stats = handle.await.expect("task panicked").unwrap();
    assert_eq!(stats.successes, 1);
}

#[tokio::test]
async fn with_client_uses_provided_client() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    let client = Client::new();
    let exec = Executor::with_client(
        RequestSettings::default(),
        ExecutionMode::FullTracking,
        client,
    );

    let stats = exec.run(make_request(&server.url()), None).await.unwrap();
    assert_eq!(stats.successes, 1);
}

// ResponseData fields

#[tokio::test]
async fn response_data_has_correct_status_and_latency() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(404)
        .create_async()
        .await;

    let (tx, mut rx) = mpsc::channel(4);
    default_executor()
        .run(make_request(&server.url()), Some(tx))
        .await
        .unwrap();

    let resp = rx.recv().await.unwrap();
    assert_eq!(resp.status.as_u16(), 404);
    // Latency should be non-zero but sane (< 5 s in CI)
    assert!(resp.response_time < Duration::from_secs(5));
}

#[tokio::test]
async fn set_cookie_headers_are_parsed() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/")
        .with_status(200)
        .with_header("set-cookie", "session=abc123; Path=/")
        .create_async()
        .await;

    let (tx, mut rx) = mpsc::channel(4);
    default_executor()
        .run(make_request(&server.url()), Some(tx))
        .await
        .unwrap();

    let resp = rx.recv().await.unwrap();
    assert!(!resp.cookies.is_empty());
    assert_eq!(resp.cookies[0].name(), "session");
    assert_eq!(resp.cookies[0].value(), "abc123");
}
