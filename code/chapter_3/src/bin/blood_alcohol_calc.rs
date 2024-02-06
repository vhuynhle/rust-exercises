use std::io::stdin;

use utils::read_value;

enum Gender {
    Male,
    Female,
}

impl From<&str> for Gender {
    fn from(value: &str) -> Self {
        match value {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => panic!("Not a valid gender"),
        }
    }
}

fn main() {
    println!("Total alcohol consumed (oz):");
    let mut stdin = stdin();
    let amount: f64 = read_value(&mut stdin).expect("Cannot read alcohol amount.");

    println!("Body weight (pound):");
    let body_weight: f64 = read_value(&mut stdin).expect("Cannot read body weight.");

    println!("Gender (M/F):");
    let gender: Gender = read_value::<String>(&mut stdin)
        .expect("Cannot read gender")
        .trim()
        .to_uppercase()
        .as_str()
        .into();
    let alcohol_distribution_ratio = match gender {
        Gender::Male => 0.73,
        Gender::Female => 0.66,
    };

    println!("Time since the last drink (hours):");
    let hours: f64 = read_value(&mut stdin).expect("Cannot read time since last drink.");

    let bac = amount * 5.14 / (body_weight * alcohol_distribution_ratio) - 0.015 * hours;
    println!("Your BAC is {bac:.2}");

    const BAC_LIMIT: f64 = 0.08;
    if bac >= BAC_LIMIT {
        println!("It is not legal to drive.");
    }
}
