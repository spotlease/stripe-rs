extern crate stripe;

use std::env;
use stripe::{Customer, CustomerParams, CustomerSource};

fn main() {
//     // Create a new client
//     let secret_key = env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
//     let client = stripe::Client::new(secret_key);

//     // Create the customer
//     let customer = Customer::create(
//         &client,
//         CustomerParams {
//             email: Some("jdoe@example.org"),
//             source: Some(CustomerSource::Token("tok_189g322eZvKYlo2CeoPw2sdy")),

//             // TODO: Keep track of https://github.com/rust-lang/rust-roadmap/issues/17
//             //       so we can use default struct field value syntax eventually
//             account_balance: None,
//             business_vat_id: None,
//             coupon: None,
//             description: None,
//             metadata: None,
//             shipping: None,
//         },
//     ).unwrap();


//     // Output in a ~prettyprint format
//     println!(
//         "Customer {{
//     id: {:?},
//     created: {:?},
//     default_source: {:?},
//     email: {:?},
//     ..
// }}",
//         customer.id,
//         customer.created,
//         customer.default_source,
//         customer.email
//     );
}
