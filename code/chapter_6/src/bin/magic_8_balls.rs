use std::io::stdin;

use rand::{seq::SliceRandom, thread_rng};
use utils::read_value;

const ANSWERS: [&str; 4] = ["Yes", "No", "Maybe", "Ask again later"];

fn main() {
    let mut rng = thread_rng();
    println!("What is your question?");
    let _: String = read_value(&mut stdin()).expect("Cannot read your question.");

    println!("{}", ANSWERS.choose(&mut rng).unwrap());
}
