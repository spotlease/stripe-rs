use error::{Error, RequestError};
use reqwest;
use reqwest::Url;
use reqwest::header::Headers;
use serde;
use serde::de::DeserializeOwned;
use request::{ApiRequest};

const DEFAULT_API_URL: &'static str = "https://api.stripe.com/v1";

#[derive(Clone, Default)]
pub struct Params {
    pub stripe_account: Option<String>,
}

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    api_url: String,
    secret_key: String,
    params: Params,
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://api.stripe.com/v1/{}", &path[1..])
    }

    pub fn path_to_url(&self, path: &str) -> Url {
        Url::parse(&format!("{}/{}", self.api_url, &path[1..])).unwrap()
    }

    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        Client {
            inner: reqwest::Client::new(),
            api_url: DEFAULT_API_URL.to_owned(),
            secret_key: secret_key.into(),
            params: Params::default(),
        }
    }

    pub fn execute<T: DeserializeOwned>(&self, mut request: ApiRequest) -> Result <T, Error> {

        self.set_url(&mut request);
        self.set_headers(&mut request);

        let response = self.inner.execute(request.inner)?;
        match response.status().as_u16() {
            200 => response.json().map_err(|err| Error::from(err)),
            _ => {
                Err(match response.json() {
                    Ok(request_err) => {
                        let request_err: RequestError = request_err;
                        Error::from(request_err)
                    }
                    Err(json_err) => {
                        Error::from(json_err)
                    }
                })
            }
        }
    }

    fn set_url(&self, request: &mut ApiRequest) -> Result<(), Error> {
        use std::mem;

        let new_url = Url::parse(&format!("{}{}", &self.api_url, &request.path[1..]))
            .map_err(|err| Error::from(err))?;
        mem::replace(request.inner.url_mut(), new_url);
        Ok(())
    }

    fn set_headers(&self, request: &mut ApiRequest) {
        use reqwest::header::{Authorization, Basic, ContentType};

        let mut headers = request.inner.headers_mut();
        headers.set(Authorization(Basic {
            username: self.secret_key.clone(),
            password: None,
        }));
        headers.set(ContentType::form_url_encoded());
        if let Some(ref account) = self.params.stripe_account {
            headers.set_raw("Stripe-Account", vec![account.as_bytes().to_vec()]);
        }
    }
}
