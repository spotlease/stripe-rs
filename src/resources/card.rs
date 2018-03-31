use Currency;
use params::Metadata;

#[derive(Debug, Serialize)]
pub struct CardParams<'a> {
    pub object: &'static str,
    pub exp_month: &'a str,
    pub exp_year: &'a str,
    pub number: &'a str,
    pub name: Option<&'a str>,
    pub cvc: Option<&'a str>,
}

impl<'a> Default for CardParams<'a> {
    fn default() -> Self {
        CardParams {
            object: "card",
            exp_month: "",
            exp_year: "",
            number: "",
            name: None,
            cvc: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Check {
    Pass,
    Fail,
    Unavailable,
    Unchecked,
}

#[derive(Debug, Deserialize)]
pub enum Brand {
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Diners Club")]
    DinersClub,
    Discover,
    JCB,
    MasterCard,
    UnionPay,
    Visa,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Funding {
    Credit,
    Debit,
    Prepaid,
    Unknown
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenizationMethod {
    ApplePay,
    AndroidPay,
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub account: Option<String>,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<Check>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<Check>,
    pub brand: Brand,
    pub country: String, // Two-letter ISO code TODO: make enum
    pub currency: Option<Currency>,
    pub customer: Option<String>,
    pub cvc_check: Option<Check>,
    pub default_for_currency: Option<bool>,
    pub dynamic_last4: Option<String>,
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: String,
    pub funding: Funding,
    pub last4: String,
    pub metadata: Metadata,
    pub name: Option<String>,
    pub recipient: Option<String>,
    pub tokenization_method: Option<TokenizationMethod>,
}