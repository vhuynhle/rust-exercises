use chrono::{prelude::Utc, Datelike};
use std::io::{ErrorKind, Stdin};

pub fn read_value<T>(input: &mut Stdin) -> Result<T, std::io::Error>
where
    T: std::str::FromStr,
{
    let mut line = String::new();
    input.read_line(&mut line)?;
    line.trim()
        .parse()
        .map_err(|_| std::io::Error::new(ErrorKind::InvalidInput, "Cannot parse input"))
}

fn main() -> Result<(), std::io::Error> {
    println!("What is your current age?");
    let current_age = read_value::<i32>(&mut std::io::stdin())?;

    println!("At what age would you like to retire?");
    let retire_age = read_value::<i32>(&mut std::io::stdin())?;

    if current_age >= retire_age {
        println!("You should have been retired already!");
    } else {
        let current_year = Utc::now().year();
        println!(
            "It's {}, so you can retire in {}",
            current_year,
            current_year + (retire_age - current_age)
        );
    }

    Ok(())
}
