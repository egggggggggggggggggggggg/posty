use reqwest::{
    Body, Client, Method, Request, RequestBuilder, Url, Version,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, ops::Deref};

pub async fn make_request(
    method: Method,
    url: Url,
    headers: HeaderMap,
    body: Body,
    version: Version,
) {
    let client = Client::new();
    let req = client
        .request(method, url)
        .body(body)
        .version(version)
        .headers(headers)
        .build()
        .unwrap();
    //For expanding from the base functionality we should add some way of delayed request
    //execution.
    client.execute(req).await.unwrap();
    //creates a valid requests that can be handled later on by the client,
}
///This is for managing requests that are made. Queues up requests until the user wants to fulfill
///them.
struct RequestHandler {
    requests: Vec<Request>,
    client: Client,
    ///Limit to the amount of requests that can be sent/handled. If None means requests are all
    ///handled.
    limit: Option<usize>,
}
impl Deref for RequestHandler {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
impl RequestHandler {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
            client: Client::new(),
            limit: None,
        }
    }
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }
    pub fn remove_limit(&mut self) {
        self.limit = None;
    }
    pub async fn execute_request() {}
    ///Queues the request to be executed en masse,
    pub fn queue_request(&mut self, request: Request) {
        self.requests.push(request);
    }
    ///Executes the queued requets. Will be done on a single thread or multiple.
    pub async fn execute_queued_requests(&mut self, threaded: bool) {
        if threaded {
            std::thread::spawn(async move || {});
        } else {
            while let Some(request) = self.requests.pop() {
                let res = self.client.execute(request).await.unwrap();
            }
        }
    }
}
