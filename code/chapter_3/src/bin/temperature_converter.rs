use std::{
    fmt::Display,
    io::{stdin, ErrorKind, Write},
    str::FromStr,
};

use utils::read_value;

// ANCHOR: temperature_scale_enum
enum TemperatureScale {
    Celsius,
    Farenheit,
}
// ANCHOR_END: temperature_scale_enum

impl FromStr for TemperatureScale {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(TemperatureScale::Celsius),
            "F" => Ok(TemperatureScale::Farenheit),
            _ => Err(std::io::Error::from(ErrorKind::InvalidData)),
        }
    }
}

// ANCHOR: display_trait
impl Display for TemperatureScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureScale::Celsius => write!(f, "Celsius"),
            TemperatureScale::Farenheit => write!(f, "Farenheit"),
        }
    }
}
// ANCHOR_END: display_trait

fn main() {
    println!("Press C to convert from Fahrenheit to Celsius.");
    println!("Press F to convert from Celsius to Fahrenheit.");
    let target_scale: TemperatureScale =
        read_value::<TemperatureScale>(&mut stdin()).expect("Invalid target scale.");

    let source_scale = match target_scale {
        TemperatureScale::Celsius => TemperatureScale::Farenheit,
        TemperatureScale::Farenheit => TemperatureScale::Celsius,
    };
    print!("Enter temperature in {}: ", source_scale);
    let _ = std::io::stdout().flush();
    let source_value: f64 = read_value(&mut stdin()).expect("Cannot read source value.");

    let target_value = match target_scale {
        TemperatureScale::Celsius => (source_value - 32.0) * 5.0 / 9.0,
        TemperatureScale::Farenheit => (source_value * 9.0 / 5.0) + 32.0,
    };

    println!("The temperature in {target_scale} is {target_value:.2}.");
}
