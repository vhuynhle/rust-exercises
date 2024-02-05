use std::io::stdin;

use utils::read_value;

fn main() {
    println!("What is the order amount?");
    let amount: f64 = read_value(&mut stdin()).expect("Cannot read the order amount");

    println!("What is the state?");
    let state: String = read_value::<String>(&mut stdin())
        .expect("Cannot read the state")
        .trim()
        .to_uppercase();

    // ANCHOR: if
    if state == "WI" {
        const WI_TAX_RATE: f64 = 0.055;
        let tax = WI_TAX_RATE * amount;
        let total = tax + amount;
        println!("The subtotal is ${amount:.2}.");
        println!("The tax is ${tax:.2}.");
        println!("The total is ${total:.2}.");
    } else {
        println!("The total is ${amount:2}.");
    }
    //ANCHOR_END: if
}
