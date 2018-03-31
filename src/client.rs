use error::{Error, RequestError};
use request::ApiRequest;
use reqwest;
use reqwest::Url;
use serde::de::DeserializeOwned;

const DEFAULT_API_URL: &'static str = "https://api.stripe.com/v1";

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    api_url: Url,
    secret_key: String
}

impl Client {
    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        Client {
            inner: reqwest::Client::new(),
            api_url: Url::parse(DEFAULT_API_URL).unwrap(),
            secret_key: secret_key.into(),
        }
    }

    pub fn execute<T: DeserializeOwned>(&self, mut request: ApiRequest<T>) -> Result<T, Error> {
        self.set_url(&mut request);
        self.set_headers(&mut request);

        let mut response = self.inner.execute(request.inner)?;
        match response.status().as_u16() {
            200 => response.json().map_err(|err| Error::from(err)),
            _ => Err(match response.json() {
                Ok(request_err) => {
                    let request_err: RequestError = request_err;
                    Error::from(request_err)
                }
                Err(json_err) => Error::from(json_err),
            }),
        }
    }

    fn set_url<T: DeserializeOwned>(&self, request: &mut ApiRequest<T>) {
        use std::mem;

        let new_url = Url::parse(&format!("{}{}", &self.api_url, &request.path[1..])).unwrap();
        mem::replace(request.inner.url_mut(), new_url);
    }

    fn set_headers<T: DeserializeOwned>(&self, request: &mut ApiRequest<T>) {
        use reqwest::header::{Authorization, Basic, ContentType};

        let headers = request.inner.headers_mut();
        headers.set(Authorization(Basic {
            username: self.secret_key.clone(),
            password: None,
        }));
        headers.set(ContentType::form_url_encoded());
    }
}
