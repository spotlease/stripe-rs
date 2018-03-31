use error::ErrorCode;
use params::{List, Metadata, RangeQuery, Timestamp};
use resources::{Address, Currency, CustomerSourceParam, Refund, Source};
use request::ApiRequest;

/// The set of parameters that can be used when capturing a charge.
///
/// For more details see https://stripe.com/docs/api#charge_capture.
#[derive(Default, Serialize)]
pub struct CaptureParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

#[derive(Serialize)]
pub struct DestinationParams<'a> {
    pub account: &'a str,
    pub amount: u64,
}


/// The set of parameters that can be used when creating or updating a charge.
///
/// For more details see https://stripe.com/docs/api#create_charge and https://stripe.com/docs/api#update_charge.
#[derive(Default, Serialize)]
pub struct ChargeParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>, // NOTE: if None, Stripe assumes true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CustomerSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    All,
    AlipayAccount,
    BankAccount,
    BitcoinReceiver,
    Card,
}

#[derive(Serialize)]
pub struct SourceFilter {
    pub object: SourceType,
}

impl SourceFilter {
    pub fn all() -> SourceFilter {
        SourceFilter { object: SourceType::All }
    }
    pub fn alipay() -> SourceFilter {
        SourceFilter { object: SourceType::AlipayAccount }
    }
    pub fn bank() -> SourceFilter {
        SourceFilter { object: SourceType::BankAccount }
    }
    pub fn bitcoin() -> SourceFilter {
        SourceFilter { object: SourceType::BitcoinReceiver }
    }
    pub fn card() -> SourceFilter {
        SourceFilter { object: SourceType::Card }
    }
}

/// The set of parameters that can be used when listing charges.
///
/// For more details see https://stripe.com/docs/api#list_charges
#[derive(Default, Serialize)]
pub struct ChargeListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// Creates a new charge.
///
/// For more details see https://stripe.com/docs/api#create_charge.
pub fn create(params: ChargeParams) -> ApiRequest<Charge> {
    ApiRequest::post("charges")
        .with_body_params(params)
}

/// Retrieves the details of a charge.
///
/// For more details see https://stripe.com/docs/api#retrieve_charge.
pub fn retrieve(charge_id: &str) -> ApiRequest<Charge> {
    ApiRequest::get(&format!("/charges/{}", charge_id))
}

/// Updates a charge's properties.
///
/// For more details see https://stripe.com/docs/api#update_charge.
pub fn update(charge_id: &str, params: ChargeParams) -> ApiRequest<Charge> {
    ApiRequest::post(&format!("/charges/{}", charge_id))
        .with_body_params(params)
}

/// Capture captures a previously created charge with capture set to false.
///
/// For more details see https://stripe.com/docs/api#charge_capture.
pub fn capture(charge_id: &str, params: CaptureParams) -> ApiRequest<Charge> {
    ApiRequest::post(&format!("/charges/{}/capture", charge_id))
        .with_body_params(params)
}

/// List all charges.
///
/// For more details see https://stripe.com/docs/api#list_charges.
pub fn list(params: ChargeListParams) -> ApiRequest<List<Charge>> {
    ApiRequest::get("/charges")
        .with_body_params(params)
}