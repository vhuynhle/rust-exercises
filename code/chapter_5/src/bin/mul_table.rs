fn main() {
    // Print the header
    print!("{:4}", ' ');
    for i in 0..13 {
        print!("{i:4}");
    }
    println!();

    // Print the rows
    for i in 0..13 {
        print!("{:4}", i);
        for j in 0..13 {
            print!("{:4}", i * j);
        }
        println!();
    }
}
