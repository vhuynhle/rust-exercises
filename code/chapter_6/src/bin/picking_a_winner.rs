use rand::seq::SliceRandom;
use std::io::Write;

fn main() {
    let mut line = String::new();
    let mut names = vec![];
    loop {
        line.clear();
        print!("Enter a name: ");
        let _ = std::io::stdout().flush();

        if std::io::stdin().read_line(&mut line).is_err() {
            break;
        }

        let name = line.trim().to_string();
        if name.is_empty() {
            break;
        }

        names.push(name);
    }

    if names.is_empty() {
        println!("There is no name, thus there is no winner.");
    }

    let mut rng = rand::thread_rng();
    let winner = names.choose(&mut rng).unwrap();
    println!("The winner is .. {}.", winner);
}
