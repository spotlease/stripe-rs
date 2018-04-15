use client::Client;
use error::Error;
use params::{List, Metadata, RangeQuery, Timestamp};
use resources::{Address, CardParams, Currency, Deleted, Discount, Source, Subscription};

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerShippingDetails {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CustomerSourceParam<'a> {
    Id(&'a str),
    Token(&'a str),
    Card(CardParams<'a>),
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
    #[serde(skip_serializing_if = "Option::is_none", rename="default_source")]
    pub default_source_id: Option<&'a str>,
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

impl Customer {
    /// Creates a new customer.
    ///
    /// For more details see https://stripe.com/docs/api#create_customer.
    pub fn create(client: &Client, params: CustomerParams) -> Result<Customer, Error> {
        client.post_with_params("/customers", params)
    }

    /// Retrieves the details of a customer.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_customer.
    pub fn retrieve(client: &Client, customer_id: &str) -> Result<Customer, Error> {
        client.get(&format!("/customers/{}", customer_id))
    }

    /// Updates a customer's properties.
    ///
    /// For more details see https://stripe.com/docs/api#update_customer.
    pub fn update(client: &Client, customer_id: &str, params: CustomerParams) -> Result<Customer, Error> {
        client.post_with_params(&format!("/customers/{}", customer_id), params)
    }

    /// Deletes a customer.
    ///
    /// For more details see https://stripe.com/docs/api#delete_customer.
    pub fn delete(client: &Client, customer_id: &str) -> Result<Deleted, Error> {
        client.delete(&format!("/customers/{}", customer_id))
    }

    /// List customers.
    ///
    /// For more details see https://stripe.com/docs/api#list_customers.
    pub fn list(client: &Client, params: CustomerListParams) -> Result<List<Customer>, Error> {
        client.get_with_params("/customers", params)
    }

    pub fn attach_source(client: &Client, customer_id: &str, source: CustomerSourceParam) -> Result<Source, Error> {
        #[derive(Debug, Serialize)]
        struct Params<'a> {
            source: CustomerSourceParam<'a>,
        }

        client.post_with_params(
            &format!("/customers/{}/sources", customer_id),
            Params { source: source },
        )
    }

    pub fn detach_source(client: &Client, customer_id: &str, source_id: &str) -> Result<Deleted, Error> {
        client.delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }
}
