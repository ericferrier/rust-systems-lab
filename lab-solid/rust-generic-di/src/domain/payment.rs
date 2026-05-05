pub trait PaymentProcessor {
    fn process(&self, amount: f64);
}