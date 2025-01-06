use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
