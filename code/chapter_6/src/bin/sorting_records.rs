use chapter_6::record::Record;

fn main() {
    let mut records: Vec<Record> = std::io::stdin()
        .lines()
        .map(|res| {
            res.expect("Error reading input")
                .parse::<Record>()
                .expect("Error parsing record")
        })
        .collect();

    records.sort_by(|r1, r2| r1.last_name.cmp(&r2.last_name));
    for record in records {
        println!("{:?}", record);
    }
}
