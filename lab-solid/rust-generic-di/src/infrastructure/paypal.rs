use crate::domain::payment::PaymentProcessor;

pub struct Paypal;

impl PaymentProcessor for Paypal {
    fn process(&self, amount: f64) {
        println!("Processing ${} with PayPal", amount);
    }
}