use std::io::stdin;

use utils::read_value;

fn main() {
    println!("BMI calculator.");

    println!("Enter mass (kg):");
    let mass: f64 = read_value(&mut stdin()).expect("Cannot read mass.");

    println!("Enter height (m):");
    let height: f64 = read_value(&mut stdin()).expect("Cannot read height.");

    let bmi = mass / (height * height);

    println!("Your BMI is {bmi:.2}");
    if bmi < 18.5 {
        println!("You are underweight. You should see your doctor.");
    } else if bmi > 25.0 {
        println!("You are overweight. You should see your doctor.");
    } else {
        println!("You are within the ideal range.");
    }
}
