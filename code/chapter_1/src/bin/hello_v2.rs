// ANCHOR: use_rand
use rand::{seq::SliceRandom, thread_rng};
// ANCHOR_END: use_rand
use std::io;

// ANCHOR: phrasebook
const GREETINGS: &[&str] = &[
    "Hi",
    "Hi there",
    "Hey",
    "Hello",
    "Hello, nice to meet you",
    "I've heard so much about you",
];
// ANCHOR_END: phrasebook

fn main() -> io::Result<()> {
    println!("What is your name?");
    let name = io::stdin().lines().next().expect("No line entered")?;

    // ANCHOR: randomized_greetings
    let mut rng = thread_rng();
    println!("{}, {}!", GREETINGS.choose(&mut rng).unwrap(), name);
    // ANCHOR_END: randomized_greetings

    Ok(())
}
