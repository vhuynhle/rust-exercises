use chapter_6::record::Record;

fn main() {
    let mut lines = std::io::stdin().lines();
    let search_string = lines
        .next()
        .expect("No search string provided")
        .expect("Error reading search string")
        .trim()
        .to_string();

    let records: Vec<Record> = lines
        .map(|res| {
            res.expect("Error reading input")
                .parse::<Record>()
                .expect("Error parsing record")
        })
        .collect();

    for record in records
        .iter()
        .filter(|r| r.first_name.contains(&search_string) || r.last_name.contains(&search_string))
    {
        println!("{:?}", record);
    }
}
