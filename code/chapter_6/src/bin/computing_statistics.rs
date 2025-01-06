use std::io::Write;

fn main() {
    let mut line = String::new();
    let mut numbers: Vec<f64> = vec![];

    loop {
        line.clear();
        print!("Enter a number: ");
        let _ = std::io::stdout().flush();
        if std::io::stdin().read_line(&mut line).is_err() {
            eprintln!("Read error");
            std::process::exit(1);
        }

        match line.trim() {
            "done" | "" => break,
            trimmed_line => match trimmed_line.parse::<f64>() {
                Ok(number) if number.is_finite() => numbers.push(number),
                _ => eprintln!("Ignored invalid number"),
            },
        }
    }

    if numbers.is_empty() {
        println!("No numbers entered");
        std::process::exit(0);
    }

    let sum: f64 = numbers.iter().sum();
    let average = sum / numbers.len() as f64;
    let min = numbers
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = numbers
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let sum_of_sqr = numbers
        .iter()
        .map(|x| -> f64 {
            let d = x - average;
            d * d
        })
        .sum::<f64>();
    let population_variance = sum_of_sqr / numbers.len() as f64;
    let population_stddev = population_variance.sqrt();
    let sample_variance = if numbers.len() == 1 {
        0.0
    } else {
        sum_of_sqr / (numbers.len() - 1) as f64
    };
    let sample_stddev = sample_variance.sqrt();

    println!("The average is {:.2}", average);
    println!("The minimum is {:.2}", min);
    println!("The maximum is {:.2}", max);
    println!("The population standard deviation is {:.2}", population_stddev);
    println!("The sample standard deviation is {:.2}", sample_stddev);
}
