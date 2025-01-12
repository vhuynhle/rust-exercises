use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
struct Resp {
    message: String,
    number: usize,
    people: Vec<Astronaut>,
}

#[derive(Deserialize)]
struct Astronaut {
    name: String,
    craft: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    const URL: &str = "http://api.open-notify.org/astros.json";
    let resp = reqwest::blocking::get(URL)?.text()?;

    let resp: Resp = serde_json::from_str(&resp).expect("Unexpected JSON content");
    if resp.message != "success" {
        eprintln!("Failed: {}", resp.message);
    }

    let noun = if resp.number > 1 { "people" } else { "person" };
    println!("There are {} {} in space right now:", { resp.number }, noun);
    println!("{:<30} | Craft", "Name");
    println!("{:-<31}|------", "");
    for astronaut in resp.people.iter() {
        println!("{:<30} | {}", astronaut.name, astronaut.craft);
    }

    Ok(())
}
