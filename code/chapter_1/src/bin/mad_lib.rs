use rand::{rngs::ThreadRng, seq::SliceRandom};

// ANCHOR: main
fn main() {
    let mut rng = rand::thread_rng();
    let noun = random_choice(NOUNS, &mut rng);
    let verb = random_choice(VERBS, &mut rng);
    let adjective = random_choice(ADJECTIVES, &mut rng);
    let adverb = random_choice(ADVERBS, &mut rng);

    println!(
        "Do you {} your {} {} {}? That's hilarious!",
        verb, adjective, noun, adverb
    );
}
// ANCHOR_END: main

// ANCHOR: database
const NOUNS: &[&str] = &["dog", "cat", "elephant", "goldfish", "python"];
const VERBS: &[&str] = &["walk", "touch", "kiss", "run", "feed"];
const ADJECTIVES: &[&str] = &["blue", "hot", "cool", "soft", "hard", "dry", "wet"];
const ADVERBS: &[&str] = &["quickly", "lazily", "happily", "hungrily"];
// ANCHOR_END: database

// ANCHOR: fn_random_choice
fn random_choice<'a>(choices: &[&'a str], rng: &mut ThreadRng) -> &'a str {
    choices.choose(rng).unwrap()
}
// ANCHOR_END: fn_random_choice
