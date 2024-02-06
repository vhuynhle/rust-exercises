use std::io::stdin;

use utils::read_value;

fn main() {
    let mut count: usize = 0;
    let mut min_value = i32::MAX;
    let mut max_value = i32::MIN;

    let mut stdin = stdin();
    loop {
        println!("Enter number #{}", count + 1);
        let value: i32 = match read_value::<i32>(&mut stdin) {
            Ok(value) => value,
            _ => break,
        };
        min_value = min_value.min(value);
        max_value = max_value.max(value);
        count += 1;
    }

    if count == 0 || min_value == max_value {
        return;
    }
    println!("The largest number is {max_value}");
}
