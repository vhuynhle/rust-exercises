use std::io;

fn main() -> io::Result<()> {
    println!("What is the quote?");
    let mut inputs = io::stdin().lines();
    let quote = inputs.next().expect("No quote provided")?;

    println!("Who said it?");
    let author = inputs.next().expect("No author provided")?;

    println!("{} says, \"{}\"", author, quote);

    Ok(())
}
