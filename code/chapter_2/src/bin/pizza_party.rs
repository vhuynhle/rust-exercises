use std::io::stdin;
use utils::read_value;

// ANCHOR: pluralize
fn print_pieces(num: u64) -> String {
    if num > 1 {
        format!("{} pieces", num)
    } else {
        format!("{} piece", num)
    }
}
// ANCHOR_END: pluralize

fn main() -> std::io::Result<()> {
    println!("How many people?");
    let npeople: u64 = read_value(&mut stdin())?;

    println!("How many pizza?");
    let npizzas: u64 = read_value(&mut stdin())?;

    println!("How many slices are there in a pizza?");
    let nslices_per_pizza: u64 = read_value(&mut stdin())?;

    // ANCHOR: calc
    let ntotal_slices = nslices_per_pizza * npizzas;
    let slices_per_person = ntotal_slices / npeople;
    let remainder = ntotal_slices % npeople;
    // ANCHOR_END: calc

    println!(
        "Each person get {} of pizza.",
        print_pieces(slices_per_person)
    );
    println!("There are {} leftover pieces.", print_pieces(remainder));

    Ok(())
}
