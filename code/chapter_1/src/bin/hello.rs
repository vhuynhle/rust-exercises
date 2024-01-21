use std::io;

fn main() -> io::Result<()>{
    println!("What is your name?");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();

    println!("Hello, {name}!");

    Ok(())
}
