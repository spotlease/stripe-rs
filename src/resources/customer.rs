use error::Error;
use resources::{Address, CardParams, Currency, Discount, Source, Subscription, Card, BankAccount};
use params::{List, Metadata, RangeQuery, Timestamp};
use request::ApiRequest;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CustomerSourceParam<'a> {
    Token(&'a str),
    Card(CardParams<'a>)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerShippingDetails {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

/// The set of parameters that can be used when creating or updating a customer.
///
/// For more details see https://stripe.com/docs/api#create_customer and https://stripe.com/docs/api#update_customer.
#[derive(Default, Serialize)]
pub struct CustomerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CustomerSourceParam<'a>>,
}

/// The set of parameters that can be used when listing customers.
///
/// For more details see https://stripe.com/docs/api#list_customers
#[derive(Default, Serialize)]
pub struct CustomerListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "object")]
pub enum CustomerSource {
    #[serde(rename = "bank_account")]
    BankAccount(BankAccount),
    #[serde(rename = "card")]
    Card(Card)
}

/// The resource representing a Stripe customer.
///
/// For more details see https://stripe.com/docs/api#customers.
#[derive(Debug, Deserialize)]
pub struct Customer {
    pub id: String,
    pub account_balance: i64,
    pub business_vat_id: Option<String>,
    pub created: u64,
    pub currency: Option<Currency>,
    pub default_source: Option<String>,
    pub delinquent: bool,
    pub desc: Option<String>,
    pub discount: Option<Discount>,
    pub email: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub shipping: Option<CustomerShippingDetails>,
    pub sources: List<Source>,
    pub subscriptions: List<Subscription>,
}

/// Creates a new customer.
///
/// For more details see https://stripe.com/docs/api#create_customer.
pub fn create(params: CustomerParams) -> ApiRequest {
    ApiRequest::post("/customers")
        .with_body_params(params)
}

/// Retrieves the details of a customer.
///
/// For more details see https://stripe.com/docs/api#retrieve_customer.
pub fn retrieve(customer_id: &str) -> ApiRequest {
    ApiRequest::get(&format!("/customers/{}", customer_id))
}

/// Updates a customer's properties.
///
/// For more details see https://stripe.com/docs/api#update_customer.
pub fn update(customer_id: &str, params: CustomerParams) -> ApiRequest {
    ApiRequest::post(&format!("/customers/{}", customer_id))
        .with_body_params(params)
}

/// Deletes a customer.
///
/// For more details see https://stripe.com/docs/api#delete_customer.
pub fn delete(customer_id: &str) -> ApiRequest {
    ApiRequest::delete(&format!("/customers/{}", customer_id))
}

/// List customers.
///
/// For more details see https://stripe.com/docs/api#list_customers.
pub fn list(params: CustomerListParams) -> ApiRequest {
    ApiRequest::get("/customers")
        .with_body_params(params)
}

/// Attach a source.
///
/// For more details see https://stripe.com/docs/api/curl#attach_source.
pub fn attach_source(customer_id: &str, source: CustomerSourceParam) -> ApiRequest {
    #[derive(Debug, Serialize)]
    struct Params<'a> {
        source: CustomerSourceParam<'a>
    }

    ApiRequest::post(&format!("/customers/{}/sources", customer_id))
        .with_body_params(Params {
            source: source
        })
}

/// Detach a source.
///
/// For more details see https://stripe.com/docs/api/curl#detach_source.
pub fn detach_source(customer_id: &str, source_id: &str) -> ApiRequest {
    ApiRequest::delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
}