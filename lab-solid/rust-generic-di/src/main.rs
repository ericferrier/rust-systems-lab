mod domain;
mod infrastructure;
mod application;

use application::checkout_service::CheckoutService;
use infrastructure::stripe::Stripe;
use infrastructure::paypal::Paypal;
use infrastructure::mock::Mock;

fn main() {
    // Compile-time dependency injection

    let stripe_service = CheckoutService::new(Stripe);
    stripe_service.checkout(100.0);

    let paypal_service = CheckoutService::new(Paypal);
    paypal_service.checkout(50.0);

    let mock_service = CheckoutService::new(Mock);
    mock_service.checkout(75.0);
}