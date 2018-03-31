use error::Error;
use params::{List, Metadata, RangeQuery, Timestamp};
use resources::{Currency, Discount, Plan};

/// The set of parameters that can be used when creating or updating an invoice.
///
/// For more details see https://stripe.com/docs/api#create_invoice, https://stripe.com/docs/api#update_invoice.
#[derive(Default, Serialize)]
pub struct InvoiceParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgiven: Option<bool>,
}

#[derive(Default, Serialize)]
pub struct InvoiceLineItemParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<bool>,
}

/*
#[derive(Serialize)]
pub struct InvoiceListLinesParams {
    #[serde(skip_serializing_if = "Option::is_none")] pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub starting_after: Option<String>,

    ..
}
*/

#[derive(Debug, Default, Serialize)]
pub struct InvoiceUpcomingParams<'a> {
    pub customer: &'a str,   // this is a required param
    #[serde(skip_serializing_if = "Option::is_none")] pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub subscription_items: Option<SubscriptionItemParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub subscription_prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub subscription_proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")] pub subscription_tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub subscription_trial_end: Option<Timestamp>,
}

#[derive(Debug, Serialize)]
pub struct SubscriptionItemParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

/// Period is a structure representing a start and end dates.
#[derive(Debug, Deserialize)]
pub struct Period {
    pub start: Timestamp,
    pub end: Timestamp,
}

/// The resource representing a Stripe invoice line item.
///
/// For more details see https://stripe.com/docs/api#invoice_line_item_object.
#[derive(Debug, Deserialize)]
pub struct InvoiceLineItem {
    pub id: String,
    pub amount: i64,
    pub currency: Currency,
    pub description: Option<String>,
    pub discountable: bool,
    pub livemode: bool,
    pub metadata: Metadata,
    pub period: Period,
    pub plan: Option<Plan>,
    pub proration: bool,
    pub quantity: Option<u64>,
    pub subscription: Option<String>,
    pub subscription_item: Option<String>,
    #[serde(default)]
    // NOTE: Missing in response to InvoiceLineItem create
    #[serde(rename = "type")]
    pub item_type: String, // (invoiceitem, subscription)
}

/// The resource representing a Stripe invoice.
///
/// For more details see https://stripe.com/docs/api#invoice_object.
#[derive(Debug, Deserialize)]
pub struct Invoice {
    pub id: Option<String>, // id field is not present when retrieving upcoming invoices
    pub amount_due: u64,
    pub application_fee: Option<u64>,
    pub attempt_count: u64,
    pub attempted: bool,
    pub charge: Option<String>,
    pub closed: bool,
    pub currency: Currency,
    pub customer: String,
    pub date: Timestamp,
    pub description: Option<String>,
    pub discount: Option<Discount>,
    pub ending_balance: Option<i64>,
    pub forgiven: bool,
    pub lines: List<InvoiceLineItem>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub next_payment_attempt: Option<Timestamp>,
    pub paid: bool,
    pub period_end: Timestamp,
    pub period_start: Timestamp,
    pub receipt_number: Option<String>,
    pub starting_balance: i64,
    pub statment_descriptor: Option<String>,
    pub subscription: Option<String>,
    pub subscription_proration_date: Option<Timestamp>,
    pub subtotal: i64,
    pub tax: Option<i64>,
    pub tax_percent: Option<f64>,
    pub total: i64,
    pub webhooks_delivered_at: Option<Timestamp>,
}

#[derive(Default, Serialize)]
pub struct InvoiceListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
}

// impl Invoice {
//     /// Creates a new invoice.
//     ///
//     /// For more details see https://stripe.com/docs/api#create_invoice.
//     pub fn create(client: &Client, params: InvoiceParams) -> Result<Invoice, Error> {
//         client.post_with_params("/invoices", params)
//     }

//     /// Retrieves the details of an invoice.
//     ///
//     /// For more details see https://stripe.com/docs/api#retrieve_invoice.
//     pub fn retrieve(client: &Client, invoice_id: &str) -> Result<Invoice, Error> {
//         client.get(&format!("/invoices/{}", invoice_id))
//     }

//     // TODO: Implement InvoiceListLinesParams
//     // pub fn get_lines(client: &Client, invoice_id: &str, params: InvoiceListLinesParams) -> Result<List<InvoiceLineItem>, Error> {
//     //     client.get(&format!("/invoices/{}/lines", invoice_id))
//     // }

//     /// Retrieves the details of an upcoming invoice_id
//     ///
//     /// For more details see https://stripe.com/docs/api#upcoming_invoice
//     pub fn upcoming(client: &Client, params: InvoiceUpcomingParams) -> Result<Invoice, Error> {
//         client.get_with_params("/invoices/upcoming", params)
//     }

//     /// Pays an invoice.
//     ///
//     /// For more details see https://stripe.com/docs/api#pay_invoice.
//     pub fn pay(client: &Client, invoice_id: &str) -> Result<Invoice, Error> {
//         client.post(&format!("/invoices/{}/pay", invoice_id))
//     }

//     /// Updates an invoice.
//     ///
//     /// For more details see https://stripe.com/docs/api#update_invoice.
//     pub fn update(client: &Client, invoice_id: &str, params: InvoiceParams) -> Result<Invoice, Error> {
//         client.post_with_params(&format!("/invoices/{}", invoice_id), &params)
//     }

//     /// Lists all invoices.
//     ///
//     /// For more details see https://stripe.com/docs/api#list_invoices.
//     pub fn list(client: &Client, params: InvoiceListParams) -> Result<List<Invoice>, Error> {
//         client.get_with_params("/invoices", params)
//     }
// }

// impl InvoiceLineItem {
//     /// Creates an invoice line item.
//     ///
//     /// For more details see https://stripe.com/docs/api#invoice_line_item_object
//     pub fn create(client: &Client, params: InvoiceLineItemParams) -> Result<InvoiceLineItem, Error> {
//         client.post_with_params("/invoiceitems", &params)
//     }
// }
