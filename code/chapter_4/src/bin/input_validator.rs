use std::io::stdin;

use utils::read_value;

fn validate_name(name: &str) -> bool {
    name.trim().len() >= 2
}

fn validate_employee_id(id: &str) -> bool {
    let id: Vec<char> = id.chars().collect();

    if id.len() != "XX-DDDD".len() {
        return false;
    }

    id[0..2].iter().all(char::is_ascii_uppercase)
        && id[2] == '-'
        && id[3..].iter().all(char::is_ascii_digit)
}

fn validate_zip_code(zip: &str) -> bool {
    !zip.is_empty() && zip.chars().all(|ch| ch.is_ascii_digit())
}

fn main() {
    let mut stdin = stdin();
    println!("Enter the first name:");
    let first_name: String = read_value(&mut stdin).expect("Failed to read first name.");

    println!("Enter the last name:");
    let last_name: String = read_value(&mut stdin).expect("Failed to read last name");

    println!("Enter the ZIP code:");
    let zip_code: String = read_value(&mut stdin).expect("Failed to read the zip code.");

    println!("Enter an employee ID:");
    let id: String = read_value(&mut stdin).expect("Failed to read employee ID.");

    let mut all_ok = true;
    if !validate_name(&first_name) {
        println!("'{first_name}' is not a valid first name. It is too short.");
        all_ok = false;
    }

    if !validate_name(&last_name) {
        println!("'{last_name}' is not a valid last name. It is too short.");
        all_ok = false;
    }

    if !validate_zip_code(&zip_code) {
        println!("ZIP code must be numeric.");
        all_ok = false;
    }

    if !validate_employee_id(&id) {
        println!("'{id}' is not a valid id.");
        all_ok = false;
    }

    if all_ok {
        println!("There were no errors found.");
    }
}
