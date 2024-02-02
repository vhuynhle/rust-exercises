use utils::read_value;

// ANCHOR: constants
const FT_TO_M: f64 = 0.3048;
// ANCHOR_END: constants

fn main() -> std::io::Result<()> {
    println!("What is the length of the room in feet?");
    let length_ft: f64 = read_value(&mut std::io::stdin())?;

    println!("What is the width of the room in feet?");
    let width_ft: f64 = read_value(&mut std::io::stdin())?;

    println!(
        "You entered dimensions of {} feet by {} feet.",
        length_ft, width_ft
    );

    // ANCHOR: area_calc
    let area_ft2 = length_ft * width_ft;

    let length_m = length_ft * FT_TO_M;
    let width_m = width_ft * FT_TO_M;
    let area_m2 = length_m * width_m;
    // ANCHOR_END: area_calc

    println!(
        "The area is\n{} square feet\n{} square meters",
        area_ft2, area_m2
    );

    Ok(())
}
