extern crate stripe;
use stripe::resources::customer;

#[test]
pub fn test_create_customer() {
    customer::create(customer::CustomerParams {
        account_balance: None,
        business_vat_id: None,
        coupon: None,
        description: None,
        email: None,
        metadata: None,
        shipping: None,
        source: None,
    });
}