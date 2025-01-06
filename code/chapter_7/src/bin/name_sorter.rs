use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
    process::exit,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        let prog_name = if args.is_empty() {
            "<program name>"
        } else {
            &args[0]
        };

        eprintln!("Usage:\n\t{} <input file> <output file>", prog_name);
        exit(1);
    }

    let input_file_name = &args[1];
    if let Ok(lines) = read_lines(input_file_name) {
        let mut lines: Vec<String> = lines.map(|res| res.expect("Error reading lines")).collect();
        lines.sort();

        let output_file_name = &args[2];
        // To overwrite an existing file, use File::create instead of File::create_new
        let output_file =
            File::create_new(output_file_name).expect("Couldn't create file to write");
        let mut outbuffer = BufWriter::new(output_file);
        for line in &lines {
            outbuffer
                .write_all(line.as_bytes())
                .expect("Error writing to file");
            outbuffer
                .write_all(&[b'\n'; 1])
                .expect("Error writing to file");
        }
    }
}
