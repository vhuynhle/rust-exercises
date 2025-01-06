use std::str::FromStr;

use chapter_7::read_lines;

struct Employee {
    last_name: String,
    first_name: String,
    salary: u32,
}

#[derive(Debug)]
enum EmployeeParseError {
    WrongNumberOfFields,
    EmptyLastName,
    EmptyFirstName,
    InvalidPostCode,
}

impl FromStr for Employee {
    type Err = EmployeeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(',').map(|field| field.trim()).collect();
        if fields.len() != 3 {
            return Err(EmployeeParseError::WrongNumberOfFields);
        }

        if fields[0].is_empty() {
            return Err(EmployeeParseError::EmptyLastName);
        }

        if fields[1].is_empty() {
            return Err(EmployeeParseError::EmptyFirstName);
        }

        let salary = match fields[2].parse::<u32>() {
            Ok(number) => number,
            _ => return Err(EmployeeParseError::InvalidPostCode),
        };

        Ok(Employee {
            last_name: fields[0].to_string(),
            first_name: fields[1].to_string(),
            salary,
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage:\n\t<program name> <input file>");
        std::process::exit(1);
    }

    println!("{:16}{:16}{:10}", "Last", "First", "Salary");
    println!("--------------------------------------");

    let input_file = &args[1];
    for line in read_lines(input_file)
        .expect("Cannot open file to read")
        .map(|r| r.expect("Error reading line"))
    {
        if line.trim().is_empty() {
            continue;
        }

        let employee: Employee = line.parse().expect("Unable to parse employee");
        println!(
            "{:16}{:16}{:<10}",
            employee.last_name, employee.first_name, employee.salary
        );
    }
}
