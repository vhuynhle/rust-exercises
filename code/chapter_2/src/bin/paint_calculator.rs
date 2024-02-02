use std::io::stdin;

use utils::read_value;

// ANCHOR: pluralize
fn pluralize(word: &str) -> String {
    match word {
        "foot" => "feet".to_string(),
        _ => word.to_owned() + "s",
    }
}
// ANCHOR_END: pluralize

// ANCHOR: find_noun_form
trait HasOne {
    const ONE: Self;
}

impl HasOne for u64 {
    const ONE: u64 = 1;
}

impl HasOne for f64 {
    const ONE: f64 = 1.0;
}

fn find_noun_form<T>(count: T, word: &str) -> String
where
    T: HasOne + std::cmp::PartialOrd,
{
    if count > T::ONE {
        pluralize(word)
    } else {
        word.to_string()
    }
}
// ANCHOR_END: find_noun_form

fn main() -> std::io::Result<()> {
    println!("What the the length of the room (in ft)?");
    let length_ft: f64 = read_value(&mut stdin())?;

    println!("What is the width of the room in ft?");
    let width_ft: f64 = read_value(&mut stdin())?;

    println!("How much paint is needed to paint 1 square feet?");
    let paint_per_ft2: f64 = read_value(&mut stdin())?;

    // ANCHOR: calc
    let area_ft2 = length_ft * width_ft;
    let required_paint: u64 = (area_ft2 / paint_per_ft2).ceil() as u64;
    // ANCHOR_END: calc

    // ANCHOR: print
    println!(
        "You will need to purchase {} {} of paint to cover {} square {}.",
        required_paint,
        find_noun_form(required_paint, "gallons"),
        area_ft2,
        find_noun_form(area_ft2, "foot")
    );
    // ANCHOR_END: print

    Ok(())
}
