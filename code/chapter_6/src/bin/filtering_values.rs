fn main() {
    println!("Enter a list of numbers, separated by spaces:");
    let even_numbers: Vec<i32> = std::io::stdin()
        .lines()
        .next()
        .expect("No input provided")
        .expect("stdin error")
        .split_whitespace()
        .map(|token| token.parse::<i32>().expect("Not a valid integer"))
        .filter(|x| x % 2 == 0)
        .collect();

    let even_number_strs: Vec<String> = even_numbers.iter().map(|num| num.to_string()).collect();

    println!("The even numbers are {}.", even_number_strs.join(" "));
}
