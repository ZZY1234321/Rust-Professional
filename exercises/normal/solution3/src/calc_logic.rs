pub fn new_birthday_probability(n: u32) -> f64 {
    let mut numerator: f64 = 1.0;
    let mut denominator: f64 = 1.0;
    
    for i in (365 - n + 1)..=365 {
        numerator *= i as f64;
    }
    for _ in 0..n {
        denominator *= 365 as f64;
    }

    1.0 - numerator / denominator
}