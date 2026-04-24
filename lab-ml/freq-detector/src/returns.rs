pub fn to_log_returns(prices: &[f32]) -> Vec<f32> {
    let mut returns = Vec::with_capacity(prices.len().saturating_sub(1));

    for i in 1..prices.len() {
        let p0 = prices[i - 1];
        let p1 = prices[i];

        // safety checks
        if p0 <= 0.0 || p1 <= 0.0 {
            continue;
        }

        let r = (p1 / p0).ln();
        returns.push(r);
    }

    returns
}