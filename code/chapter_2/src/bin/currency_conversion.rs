use std::io::stdin;

use utils::read_value;

fn main() {
    println!("How many euros are you exchanging?");
    let euros: f64 = read_value(&mut stdin()).expect("Invalid amount");

    println!("What is the exchange rate?");
    let exchange_rate: f64 = read_value(&mut stdin()).expect("Invalid exchange rate");

    // ANCHOR: calc
    let dollars = (euros * exchange_rate).ceil() / 100.0;
    // ANCHOR_END: calc

    println!("{euros} euros at exchange rate {exchange_rate} is {dollars} US dollars.");
}
