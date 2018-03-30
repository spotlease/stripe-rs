use serde;
use reqwest::{Request, Method, Url, header::Headers};
// use client::Client;
use error;

#[derive(Debug)]
pub struct ApiRequest {
    pub inner: Request,
    pub path: String
}

impl ApiRequest {
    pub fn new(method: Method, path: &str) -> Self {
        Self {
            inner: Request::new(method, Url::parse("").unwrap()),
            path: path.to_owned()
        }
    }

    pub fn get(path: &str) -> Self {
        Self::new(Method::Get, path)
    }

    pub fn post(path: &str) -> Self {
        Self::new(Method::Post, path)
    }

    pub fn delete(path: &str) -> Self {
        Self::new(Method::Delete, path)
    }
}
