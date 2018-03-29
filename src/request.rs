use serde;
use reqwest::{Request as ReqwestRequest, Method, Url, header::Headers};
use client::Client;

#[derive(Debug, Clone)]
pub struct Request<Q: serde::ser::Serialize, B: serde::ser::Serialize> {
    pub method: Method,
    pub path: String,
    pub query: Option<Q>,
    pub body: Option<B>,
    pub options: Option<RequestOptions>,
    pub stripe_account: Option<String>
}

// impl <Q: serde::ser::Serialize, B: serde::ser::Serialize> Request<Q, B> {
//     pub fn execute(self, client: &Client) {
//         let reqwest_request = ReqwestRequest::new(self.method, client.path_to_url(&self.path));


//     }
// }

// impl RequestBuilder {
//     pub fn stripe_account(&mut self, stripe_account: &str) -> &mut Self {
//         let mut headers = Headers::new();
//         headers.set_raw("Stripe-Account", vec![account.as_bytes().to_vec()]);
//         self.reqwest_builder.headers(headers);
//         &mut self
//     }
// }