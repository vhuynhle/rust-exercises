use std::io::{stdin, stdout, Write};

use utils::read_value;

fn karvonen_heart_rate(resting_hr: f64, age: i32, intensity: f64) -> f64 {
    (((220.0 - age as f64) - resting_hr) * intensity) + resting_hr
}

fn main() {
    print!("Enter resting pulse: ");
    let _ = stdout().flush();
    let resting_hr: f64 = read_value(&mut stdin()).expect("Failed to read resting pulse.");

    print!("Enter age: ");
    let _ = stdout().flush();
    let age: i32 = read_value(&mut stdin()).expect("Failed to read age.");

    // Print the header
    println!("Intensity | Rate");
    println!("----------|--------");
    for intensity in (55..=95).step_by(5) {
        println!(
            "{}%       | {:.0} bpm",
            intensity,
            karvonen_heart_rate(resting_hr, age, intensity as f64 / 100.0)
        );
    }
}
