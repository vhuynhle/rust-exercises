use std::str::FromStr;

use time::Date;

#[derive(Debug)]
pub struct Record {
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub separation_date: Option<Date>,
}

#[derive(Debug)]
pub enum ParseRecordError {
    WrongNumberOfFields,
    DateFormatError,
}

impl FromStr for Record {
    type Err = ParseRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.trim().split(',').collect();
        if fields.len() != 4 {
            return Err(ParseRecordError::WrongNumberOfFields);
        }

        let date: Option<Date> = if fields[3].is_empty() {
            None
        } else {
            let date_format = time::format_description::parse("[year]-[month]-[day]").unwrap();
            match Date::parse(fields[3], &date_format) {
                Ok(date) => Some(date),
                _ => return Err(ParseRecordError::DateFormatError),
            }
        };

        Ok(Record {
            first_name: fields[0].to_string(),
            last_name: fields[1].to_string(),
            position: fields[2].to_string(),
            separation_date: date,
        })
    }
}

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
