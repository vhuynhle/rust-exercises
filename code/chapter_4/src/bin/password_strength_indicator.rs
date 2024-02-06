use std::{
    fmt::{Debug, Display},
    io::stdin,
};

use utils::read_value;

#[derive(Debug)]
enum PasswordStrength {
    VeryWeak,
    Weak,
    Strong,
    VeryStrong,
    NotClassified,
}

impl Display for PasswordStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

fn classify_password(password: &str) -> PasswordStrength {
    let password: Vec<char> = password.chars().collect();

    if password.len() < 8 && password.iter().all(|ch| ch.is_ascii_digit()) {
        PasswordStrength::VeryWeak
    } else if password.len() < 8 && password.iter().all(|ch| ch.is_ascii_alphabetic()) {
        PasswordStrength::Weak
    } else if password.len() >= 8
        && password.iter().any(char::is_ascii_alphabetic)
        && password.iter().any(char::is_ascii_digit)
    {
        if password.iter().any(|ch| !ch.is_ascii_alphanumeric()) {
            PasswordStrength::VeryStrong
        } else {
            PasswordStrength::Strong
        }
    } else {
        PasswordStrength::NotClassified
    }
}

fn main() {
    println!("Enter a password to check it's strength");
    let password: String = read_value(&mut stdin()).expect("Failed to read password.");
    println!("Your password is '{}'", classify_password(&password));
}
