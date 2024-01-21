use std::io;

fn main() -> io::Result<()> {
    println!("Enter a string:");
    let input = io::stdin().lines().next().expect("No string entered")?;

    // ANCHOR: compute_length
    let num_bytes = input.len();
    let num_scalars = input.chars().count();
    let num_egc = unic_segment::Graphemes::new(&input).count();
    // ANCHOR_END: compute_length

    println!("Your string '{}' has:", input);
    println!("\t{} byte(s)", num_bytes);
    println!("\t{} scalar value(s)", num_scalars);
    println!("\t{} extended grapheme cluster(s).", num_egc);

    Ok(())
}
