use serde::ser::Serialize;
use serde_urlencoded;
use reqwest::{Request, Method, Url};
use reqwest::header::ContentType;

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
        let mut new_api_request = Self::new(Method::Post, path);
        new_api_request.inner.headers_mut().set(ContentType::form_url_encoded());
        new_api_request
    }

    pub fn delete(path: &str) -> Self {
        Self::new(Method::Delete, path)
    }

    pub fn with_body_params<T: Serialize>(mut self, body: T) -> Self {
        let form_body = serde_urlencoded::to_string(body).unwrap();
        *self.inner.body_mut() = Some(form_body.into());
        self
    }

    pub fn for_stripe_account(mut self, stripe_account: &str) -> Self {
        self.inner.headers_mut().set_raw("Stripe-Account", vec![stripe_account.as_bytes().to_vec()]);
        self
    }
}
