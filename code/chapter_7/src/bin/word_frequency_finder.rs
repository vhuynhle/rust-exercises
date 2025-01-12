use std::{cmp::Ordering, collections::HashMap, fs};

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

    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Cannot read file");
    let freqs = word_freqs(&contents);
    let freqs = sort_by_freq(freqs);

    print_histogram(&freqs);
}

fn word_freqs(contents: &str) -> HashMap<String, usize> {
    let mut words: HashMap<String, usize> = HashMap::new();
    let non_alphabetic = |ch: char| !ch.is_alphabetic();

    for word in contents
        .split(non_alphabetic)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
    {
        *words.entry(word).or_insert(0) += 1;
    }

    words
}

fn sort_by_freq(word_freqs: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut res: Vec<(String, usize)> = word_freqs.into_iter().collect();
    res.sort_by(|word_a, word_b| match word_a.1.cmp(&word_b.1) {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => word_a.0.cmp(&word_b.0),
        Ordering::Greater => Ordering::Less,
    });
    res
}

fn print_histogram(freqs: &[(String, usize)]) {
    if freqs.is_empty() {
        return;
    }

    let max_freq = freqs.iter().map(|(_, count)| *count).max().unwrap();
    let max_length: usize = max_freq.min(100);
    let freq_to_length = |freq: usize| (freq * max_length) / max_freq;

    println!("Distinct word count: {}", freqs.len());
    println!("Histogram:");
    for (word, freq) in freqs.iter() {
        println!("{:15} {}", word, "*".repeat(freq_to_length(*freq)));
    }
}
