use std::io;

fn main() -> io::Result<()> {
    println!("What is your name?");
    println!(
        "Hello, {}!",
        io::stdin() // Get a handle to stdin
            .lines() // iterate over input lines
            .next() // get the first one
            .expect("No line entered")? // unwrap it
            .trim() // trim whitespaces
    );
    Ok(())
}
