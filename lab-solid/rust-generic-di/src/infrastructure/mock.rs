// infrastructure/mock.rs
use crate::domain::payment::PaymentProcessor;

pub struct Mock;

impl PaymentProcessor for Mock {
    fn process(&self, amount: f64) {
        println!("Mock processing ${}", amount);
    }
}