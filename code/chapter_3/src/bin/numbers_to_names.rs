use std::{
    fmt::{Debug, Display},
    io::stdin,
};

use utils::read_value;

#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Month {
    fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None
        }
    }
}

fn main() {
    println!("Enter the month in number:");
    let month: i32 = read_value(&mut stdin()).expect("The entered value is not a number");
    let month = Month::from_i32(month).expect("Not a valid month");
    println!("The name of the month is {month}");
}
