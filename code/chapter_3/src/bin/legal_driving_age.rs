use std::io::stdin;
use utils::read_value;

const LEGAL_DRIVING_AGE: i32 = 16;
fn main() {
    println!("What is your age?");
    let age: i32 = read_value(&mut stdin()).expect("Cannot read age");
    if age >= LEGAL_DRIVING_AGE {
        println!("You are old enough to legally drive");
    } else {
        println!("You are not old enough to legally drive");
    }
}
