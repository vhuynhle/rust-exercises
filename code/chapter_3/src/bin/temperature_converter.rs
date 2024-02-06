use std::{
    fmt::Display,
    io::{stdin, ErrorKind},
};

use utils::read_value;

enum TemperatureScale {
    Celsius,
    Farenheit,
}

impl TryFrom<&str> for TemperatureScale {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" => Ok(TemperatureScale::Celsius),
            "F" => Ok(TemperatureScale::Farenheit),
            _ => Err(std::io::Error::from(ErrorKind::InvalidData)),
        }
    }
}

impl Display for TemperatureScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureScale::Celsius => write!(f, "Celsius"),
            TemperatureScale::Farenheit => write!(f, "Farenheit"),
        }
    }
}

fn main() {
    println!("Press C to convert from Fahrenheit to Celsius.");
    println!("Press F to convert from Celsius to Fahrenheit.");
    let target_scale: TemperatureScale = read_value::<String>(&mut stdin())
        .expect("Cannot read conversion direction.")
        .as_str()
        .try_into()
        .expect("Invalid target scale.");

    let source_value: f64 = read_value(&mut stdin()).expect("Cannot read source value.");

    let target_value = match target_scale {
        TemperatureScale::Celsius => (source_value - 32.0) * 5.0 / 9.0,
        TemperatureScale::Farenheit => (source_value * 9.0 / 5.0) + 32.0,
    };

    println!("The temperature in {target_scale} is {target_value:.2}.");
}
