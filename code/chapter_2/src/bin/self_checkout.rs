use std::io::stdin;

use utils::read_value;

const TAX_RATE: f64 = 0.055;

fn main() {
    let mut count = 0_u64;
    let mut subtotal = 0_f64;

    // ANCHOR: read_loop
    loop {
        println!("Enter the price of item {}", count + 1);
        let price = match read_value::<f64>(&mut stdin()) {
            Ok(price) => price,
            _ => break,
        };

        println!("Enter the quantity of item {}", count + 1);
        let quantity = match read_value::<f64>(&mut stdin()) {
            Ok(q) => q,
            _ => break,
        };

        subtotal += price * quantity;
        count += 1;
    }
    // ANCHOR_END: read_loop

    let tax = subtotal * TAX_RATE;
    let total = subtotal + tax;
    println!("Subtotal: {:.2}", subtotal);
    println!("Tax: {:.2}", tax);
    println!("Total: {:2}", total);
}
