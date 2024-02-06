use std::io::stdin;

use utils::read_value;

fn payoff_months(apr: f64, balance: f64, payment: f64) -> i64 {
    let daily_rate = apr / 365.0;
    let numerator = -(1.0 + balance / payment * (1.0 - (1.0 + daily_rate).powi(30))).log10();
    let denumerator = 30.0 * (1.0 + daily_rate).log10();
    let duration = (numerator / denumerator).ceil();
    if !duration.is_finite() || duration > 1000000000.0 * 12.0 {
        panic!("Invalid data. Not possible to payoff?");
    }

    duration as i64
}

fn main() {
    let mut stdin = stdin();
    println!("What is your balance?");
    let balance: f64 = read_value(&mut stdin).expect("Failed to read balance");

    println!("What is the APR on the card?");
    let apr: f64 = read_value(&mut stdin).expect("Failed to read APR");

    println!("What is the monthly payment you can make?");
    let payment: f64 = read_value(&mut stdin).expect("Failed to read monthly payment");

    let months = payoff_months(apr / 100.0, balance, payment);
    println!("It will take {} months to pay off this card.", months);
}
