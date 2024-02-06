



const PASSWORD: &str = "abc$123";

fn main() {
    println!("What is the password?");
    // ANCHOR: read_password
    let password = rpassword::read_password().expect("Failed to read the password.");
    // ANCHOR_END: read_password

    // ANCHOR: check
    if password == PASSWORD {
        println!("Welcome!");
    } else {
        println!("I don't know you.");
    }
    // ANCHOR_END: check
}
