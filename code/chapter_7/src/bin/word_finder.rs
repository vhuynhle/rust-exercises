use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let prog_name: &str = if args.is_empty() {
            "<program name>"
        } else {
            &args[0]
        };

        eprintln!("Usage: {} <input file>", prog_name);
        std::process::exit(1);
    }

    let from: Vec<char> = "utilize".chars().collect();
    let to = "use";
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines.map_while(Result::ok) {
            println!("{}", replace_string(&line, &from, to));
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn replace_string(input: &str, from: &[char], to: &str) -> String {
    assert!(!from.is_empty());

    let mut acc: String = String::new();
    let mut ring_buffer: VecDeque<char> = VecDeque::with_capacity(from.len());

    for ch in input.chars() {
        ring_buffer.push_back(ch);

        if ring_buffer.len() == from.len() {
            if ring_buffer
                .iter()
                .zip(from.iter())
                .all(|(ch1, ch2)| ch1 == ch2)
            {
                // matched, replacing `from` with `to`
                acc.push_str(to);
                ring_buffer.clear();
            } else {
                // No match. Extract 1 character from the buffer to make room for the next character
                acc.push(ring_buffer.pop_front().unwrap());
            }
        }
    }

    // Put the rest of the characters (no match) to the result
    for ch in ring_buffer {
        acc.push(ch);
    }

    acc
}
