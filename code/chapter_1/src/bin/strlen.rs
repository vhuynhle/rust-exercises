use std::io;

fn main() -> io::Result<()> {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    // ANCHOR: compute_length
    let num_bytes = input.len();
    let num_scalars = input.chars().count();
    let num_egc = unic_segment::Graphemes::new(input).count();
    // ANCHOR_END: compute_length

    println!("Your string '{}' has:", input);
    println!("\t{} byte(s)", num_bytes);
    println!("\t{} scalar values", num_scalars);
    println!("\t{} extended grapheme cluster(s).", num_egc);

    Ok(())
}
