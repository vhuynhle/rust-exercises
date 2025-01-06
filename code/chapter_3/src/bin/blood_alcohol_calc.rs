use std::{
    io::{stdin, Write},
    str::FromStr,
};

use utils::read_value;

// ANCHOR: parse_gender
enum Gender {
    Male,
    Female,
}

struct ParseGenderError {}

impl FromStr for Gender {
    type Err = ParseGenderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "M" | "MALE" => Ok(Gender::Male),
            "F" | "FEMALE" => Ok(Gender::Female),
            _ => Err(ParseGenderError {}),
        }
    }
}
// ANCHOR_END: parse_gender

fn main() {
    let mut stdin = stdin();

    print!("Total alcohol consumed (oz): ");
    let _ = std::io::stdout().flush();
    let alcohol_amount: f64 = read_value(&mut stdin).expect("Cannot read alcohol amount.");

    print!("Body weight (pound): ");
    let _ = std::io::stdout().flush();
    let body_weight: f64 = read_value(&mut stdin).expect("Cannot read body weight.");

    print!("Gender (M/F): ");
    let _ = std::io::stdout().flush();
    let gender: Gender = read_value::<Gender>(&mut stdin).expect("Cannot read gender");

    print!("Time since the last drink (hours): ");
    let _ = std::io::stdout().flush();
    let hours: f64 = read_value(&mut stdin).expect("Cannot read time since last drink.");

    // ANCHOR: bac_formula
    let alcohol_distribution_ratio = match gender {
        Gender::Male => 0.73,
        Gender::Female => 0.66,
    };

    let bac = alcohol_amount * 5.14 / (body_weight * alcohol_distribution_ratio) - 0.015 * hours;
    // ANCHOR_END: bac_formula

    println!("Your BAC is {bac:.2}");

    const BAC_LIMIT: f64 = 0.08;
    if bac >= BAC_LIMIT {
        println!("It is not legal to drive.");
    } else {
        println!("You can still drive with this BAC.")
    }
}
