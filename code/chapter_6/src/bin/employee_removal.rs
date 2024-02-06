use std::io::stdin;

use utils::read_value;

fn main() {
    let mut employees = vec![
        "John Smith",
        "Jackie Jackson",
        "Chris Johns",
        "Amanda Cullen",
        "Jeremy Goodwin",
    ];

    println!("There are {} employees:", employees.len());
    for &e in &employees {
        println!("{e}");
    }

    println!("Enter an employee to remove:");
    let to_be_laid_off: String =
        read_value(&mut stdin()).expect("Cannot get the name of the employee.");

    if let Some(index) = employees.iter().position(|&name| name == to_be_laid_off) {
        employees.remove(index);
    }

    println!("There are {} employees:", employees.len());
    for &e in &employees {
        println!("{e}");
    }
}
