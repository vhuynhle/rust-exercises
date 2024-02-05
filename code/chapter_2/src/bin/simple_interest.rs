use std::io::stdin;

use utils::read_value;

fn main() {
    println!("Enter the principle:");
    let principle: f64 = read_value(&mut stdin()).expect("Invalid principle amount");

    println!("Enter the rate of interest:");
    let interest: f64 = read_value(&mut stdin()).expect("Invalid interest rate");

    println!("Enter the number of years:");
    let years: f64 = read_value(&mut stdin()).expect("Invalid number of years.");

    let amount = principle * (1.0 + interest / 100.0 * years);
    println!(
        "After {} years at {}%, the investment will be worth ${}",
        years, interest, amount
    );
}
