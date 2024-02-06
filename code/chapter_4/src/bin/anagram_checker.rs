use std::io::stdin;
use utils::read_value;

fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    sort_chars(s1) == sort_chars(s2)
}

fn sort_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort();
    chars
}

fn main() {
    println!("Enter two strings and I'll tell you if they are anagrams:");
    println!("Enter the first string:");
    let s1: String = read_value(&mut stdin()).expect("Cannot read the first word");
    println!("Enter the second string:");
    let s2: String = read_value(&mut stdin()).expect("Cannot read the second word");

    if is_anagram(&s1, &s2) {
        println!("'{s1} and '{s2}' are anagrams.");
    } else {
        println!("'{s1}' and '{s2}' are not anagrams.");
    }
}
