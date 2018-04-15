use error::{Error, RequestError};
use reqwest;
use reqwest::Url;
use reqwest::Method;
use reqwest::header::Headers;
use serde;
use serde::de::DeserializeOwned;

const DEFAULT_API_URL: &'static str = "https://api.stripe.com/v1";

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    api_url: String,
    secret_key: String,
    stripe_account_id: Option<String>,
}

impl Client {
    fn url(&self, path: &str) -> Url {
        Url::parse(&format!("{}/{}", self.api_url, &path[1..])).unwrap()
    }

    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        Client {
            inner: reqwest::Client::new(),
            api_url: DEFAULT_API_URL.to_owned(),
            secret_key: secret_key.into(),
            stripe_account_id: None,
        }
    }

    /// Clones a new client with different params.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand params while using the same secret key.
    pub fn with_stripe_account_id<Str: Into<String>>(&self, account_id: Str) -> Client {
        let mut client = self.clone();
        client.stripe_account_id = Some(account_id.into());
        client
    }

    /// Sets a value for the Stripe-Account header
    ///
    /// This is recommended if you are acting as only one Account for the lifetime of the client.
    /// Otherwise, prefer `client.with(Params{stripe_account_id: "acct_ABC", ..})`.
    pub fn set_stripe_account_id<Str: Into<String>>(&mut self, account_id: Str) {
        self.stripe_account_id = Some(account_id.into());
    }

    fn request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let url = self.url(path);
        let mut request = self.inner.request(method, url);
        request.headers(self.headers());
        request
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let mut request = self.request(Method::Get, path);
        process_response(request.send()?)
    }

    pub fn get_with_params<T: DeserializeOwned, Q: serde::Serialize>(
        &self,
        path: &str,
        query_params: Q,
    ) -> Result<T, Error> {
        let mut request = self.request(Method::Get, path);
        request.query(&query_params);
        process_response(request.send()?)
    }

    pub fn post<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, Error> {
        let mut request = self.request(Method::Post, path);
        process_response(request.send()?)
    }

    pub fn post_with_params<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body_params: B,
    ) -> Result<T, Error> {
        let mut request = self.request(Method::Post, path);
        request.form(&body_params);
        process_response(request.send()?)
    }

    pub fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let mut request = self.request(Method::Delete, path);
        process_response(request.send()?)
    }

    pub fn delete_with_params<T: DeserializeOwned, Q: serde::Serialize>(
        &self,
        path: &str,
        query_params: Q,
    ) -> Result<T, Error> {
        let mut request = self.request(Method::Delete, path);
        request.query(&query_params);
        process_response(request.send()?)
    }

    fn headers(&self) -> Headers {
        use reqwest::header::{Authorization, Basic, ContentType};

        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: self.secret_key.clone(),
            password: None,
        }));
        headers.set(ContentType::form_url_encoded());
        if let Some(ref account) = self.stripe_account_id {
            headers.set_raw("Stripe-Account", vec![account.as_bytes().to_vec()]);
        }
        headers
    }
}

fn process_response<T: DeserializeOwned>(mut response: reqwest::Response) -> Result<T, Error> {
    #[derive(Debug, Deserialize)]
    struct ErrorWrapper {
        error: RequestError
    }
    match response.status().as_u16() {
        200 => response.json().map_err(|err| Error::from(err)), //TODO: error should be recorded as conversion error
        _ => {
            Err(match response.json() {
                Ok(request_err_object) => {
                    let request_err_object: ErrorWrapper = request_err_object;
                    Error::from(request_err_object.error)
                }
                Err(json_err) => Error::from(json_err),
            })
        }
    }
}
