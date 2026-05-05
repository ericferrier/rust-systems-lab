use crate::domain::payment::PaymentProcessor;

pub struct Stripe;

impl PaymentProcessor for Stripe {
    fn process(&self, amount: f64) {
        println!("Processing ${} with Stripe", amount);
    }
}