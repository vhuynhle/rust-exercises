use std::io::{stdin, stdout, Write};

use utils::read_value;

fn main() {
    let mut sum: i64 = 0;
    let mut stdin = stdin();
    let mut stdout = stdout();
    loop {
        print!("Enter a number: ");
        let _ = stdout.flush();
        if let Ok(num) = read_value::<i64>(&mut stdin) {
            sum += num;
        } else {
            break;
        }
    }

    println!("The total is {sum}");
}
