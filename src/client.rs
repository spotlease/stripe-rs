use error::{Error, RequestError};
use reqwest;
use reqwest::Url;
use reqwest::header::Headers;
use serde;
use serde::de::DeserializeOwned;

const DEFAULT_API_URL: &'static str = "https://api.stripe.com/v1";

#[derive(Clone, Default)]
pub struct Params {
    pub stripe_account: Option<String>,
}

#[derive(Clone)]
pub struct Client {
    reqwest_client: reqwest::Client,
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
            reqwest_client: reqwest::Client::new(),
            api_url: DEFAULT_API_URL.to_owned(),
            secret_key: secret_key.into(),
            params: Params::default(),
        }
    }

    //TODO: pub fn execute

    // pub fn create_reqwest_request(&self, method: reqwest::Method, path: &str) -> reqwest::Request {
    //     let url = self.path_to_url(path);
    //     reqwest::Request::new(method, url)
    // }

    /// Clones a new client with different params.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand params while using the same secret key.
    pub fn with(&self, params: Params) -> Client {
        let mut client = self.clone();
        client.params = params;
        client
    }

    /// Sets a value for the Stripe-Account header
    ///
    /// This is recommended if you are acting as only one Account for the lifetime of the client.
    /// Otherwise, prefer `client.with(Params{stripe_account: "acct_ABC", ..})`.
    pub fn set_stripe_account<Str: Into<String>>(&mut self, account_id: Str) {
        self.params.stripe_account = Some(account_id.into());
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Client::url(path);
        let mut request = self.reqwest_client.get(&url);
        request.headers(self.headers());
        process_response(request.send()?)
    }

    pub fn get_with_params<T: DeserializeOwned, P: serde::Serialize>(&self, path: &str, params: P) -> Result<T, Error> {
        let url = Client::url(path);
        let mut request = self.reqwest_client.get(&url);
        request.headers(self.headers()).query(&params);
        process_response(request.send()?)
    }

    pub fn post<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Client::url(path);
        let mut request = self.reqwest_client.post(&url);
        request.headers(self.headers());
        process_response(request.send()?)
    }

    pub fn post_with_params<T: DeserializeOwned, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Result<T, Error> {
        let url = Client::url(path);
        let mut request = self.reqwest_client.post(&url);
        request.headers(self.headers()).form(&params);
        process_response(request.send()?)
    }

    pub fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Client::url(path);
        let mut request = self.reqwest_client.post(&url);
        request.headers(self.headers());
        process_response(request.send()?)
    }

    pub fn delete_with_params<T: DeserializeOwned, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Result<T, Error> {
        let url = Client::url(path);
        let mut request = self.reqwest_client.post(&url);
        request.headers(self.headers()).query(&params);
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
        if let Some(ref account) = self.params.stripe_account {
            headers.set_raw("Stripe-Account", vec![account.as_bytes().to_vec()]);
        }
        headers
    }
}

fn process_response<T: DeserializeOwned>(mut response: reqwest::Response) -> Result<T, Error> {
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

            // let mut err = json::from_str(&body).unwrap_or_else(|err| {
            //     let mut req = ErrorObject {
            //         error: RequestError::default(),
            //     };
            //     req.error.message = Some(format!("failed to deserialize error: {}", err));
            //     req
            // });
            // err.error.http_status = status;
            // Err(Error::from(err.error))
        }
    }
}
