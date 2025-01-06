use core::panic;
use std::io::{Read, Write};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Product {
    name: String,
    price: f64,
    quantity: u64,
}

#[derive(Debug, Deserialize)]
struct ProductList {
    products: Vec<Product>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: <program name> <product json file>");
        std::process::exit(1);
    }

    let input_str = read_file_content(&args[1]);
    let product_list: ProductList =
        serde_json::from_str(&input_str).expect("JSON was not well formatted");

    print_all_products(&product_list.products);
    search_products(&product_list.products);
}

fn read_file_content(file_path: &str) -> String {
    let mut input_file = std::fs::File::open(file_path)
        .unwrap_or_else(|err| panic!("Cannot open file {}: {}", file_path, err.to_string()));
    let mut input_str = String::new();
    input_file
        .read_to_string(&mut input_str)
        .expect("Cannot read file content");
    input_str
}

fn print_all_products(products: &[Product]) {
    println!("{}", "-".repeat(50));
    println!("{:20}{:>15}{:>15}", "Name", "Price", "Quantity");
    println!("{}", "-".repeat(50));

    for product in products {
        println!(
            "{:20}{:15.2}{:15}",
            product.name, product.price, product.quantity
        );
    }
    println!("{}", "-".repeat(50));
}

fn search_products(products: &[Product]) {
    let mut product_name = String::new();
    loop {
        print!("\nWhat is product name? ");
        let _ = std::io::stdout().flush();
        product_name.clear();
        if std::io::stdin().read_line(&mut product_name).is_err() {
            break;
        }
        product_name = product_name.trim().to_lowercase();
        if product_name.is_empty() {
            break;
        }

        let mut found = false;
        for p in products {
            if p.name.to_lowercase().contains(&product_name) {
                found = true;
                println!("Name: {}", p.name);
                println!("Price: {:.2}", p.price);
                println!("Quantity on hand: {}", p.quantity);
            }
        }
        if !found {
            println!("That product was not found in our inventory");
        }
    }
}
