use std::io::stdin;

use utils::read_value;

fn main() {
    let mut stdin = stdin();

    println!("What is the principal amount?");
    let principal: f64 = read_value(&mut stdin).expect("Invalid principal amount");

    println!("What is the rate?");
    let rate: f64 = read_value(&mut stdin).expect("Invalid rate");

    println!("What is the number of years?");
    let years: f64 = read_value(&mut stdin).expect("Invalid number of years");

    println!("What is the number of times the interest is compounded per year?");
    let n: f64 = read_value(&mut stdin).expect("Invalid number");

    let amount = principal * (1.0 + rate / 100.0 / n).powf(n * years);

    println!(
        "${} invested at {}% for {} years compounded {} times per year is ${:.2}",
        principal, rate, years, n, amount
    );
}
