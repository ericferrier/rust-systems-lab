use crate::domain::payment::PaymentProcessor;

pub struct CheckoutService<P: PaymentProcessor> {
    processor: P,
}

impl<P: PaymentProcessor> CheckoutService<P> {
    pub fn new(processor: P) -> Self {
        Self { processor }
    }

    pub fn checkout(&self, amount: f64) {
        self.processor.process(amount);
    }
}