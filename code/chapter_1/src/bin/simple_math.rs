use std::io;

fn main() {
    println!("Enter the first number:");
    // ANCHOR: parse_num_1
    let num1: i32 = read_line()
        .expect("Cannot read line 1")
        .parse()
        .expect("Cannot parse the first line");
    // ANCHOR_END: parse_num_1

    // ANCHOR: parse_num_2
    println!("Enter the second number:");
    let num2: i32 = read_line()
        .expect("Cannot read line 2")
        .parse()
        .expect("Cannot parse the second line");
    // ANCHOR_END: parse_num_2

    // ANCHOR: simple_math
    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);
    println!("{} / {} = {}", num1, num2, num1 / num2);
    // ANCHOR_END: simple_math
}

// ANCHOR: read_line
fn read_line() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    Some(input.trim().to_string())
}
// ANCHOR_END: read_line
