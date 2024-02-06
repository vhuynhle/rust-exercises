use std::io::{stdin, stdout, Write};

use utils::read_value;

fn main() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!("What is the rate of return? ");
        let _ = stdout.flush();
        match read_value::<i64>(&mut stdin) {
            Ok(ror) if ror > 0 => {
                println!(
                    "It will take {} years to double your initial investment.",
                    72 / ror
                );
                break;
            }
            _ => {
                println!("That's not a valid input.");
            }
        }
    }
}
