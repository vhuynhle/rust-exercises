use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
    process::exit,
};

use rand::{thread_rng, Rng};
use utils::read_value;

fn pick_upper_bound() -> i32 {
    print!("Pick a difficulty level (1, 2, or 3): ");
    let _ = stdout().flush();
    let level: u32 = read_value(&mut stdin()).expect("Failed to read the level");
    if !(1..=3).contains(&level) {
        eprintln!("Invalid level chosen");
        exit(-1);
    }
    10_i32.pow(level)
}

fn guessing_game(target: i32) -> bool {
    println!("I have a number, what is your guess? ");
    let _ = stdout().flush();

    let mut attempts = 0;
    loop {
        attempts += 1;
        let guess: i32 = read_value(&mut stdin()).expect("Not a valid guess.");

        match guess.cmp(&target) {
            Ordering::Less => print!("Too low, guess again: "),
            Ordering::Equal => {
                println!("You got it in {attempts} guesses!");
                print!("Play again? ");
                let _ = stdout().flush();
                let play_again: String = read_value(&mut stdin()).expect("Failed to read choice.");
                if ["y", "Y", "yes", "Yes", "YES"].contains(&play_again.as_str()) {
                    return true;
                } else {
                    println!("Goodbye!");
                    return false;
                }
            }
            Ordering::Greater => print!("Too high, guess again: "),
        }
        let _ = stdout().flush();
    }
}

fn main() {
    let mut rng = thread_rng();
    println!("Let's play Guess the Number.");

    loop {
        let upper_bound = pick_upper_bound();
        let target = rng.gen_range(0..=upper_bound);
        if !guessing_game(target) {
            break;
        }
    }
}
