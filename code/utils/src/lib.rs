pub fn read_value<T>(input: &mut std::io::Stdin) -> Result<T, std::io::Error>
where
    T: std::str::FromStr,
{
    let mut line = String::new();
    input.read_line(&mut line)?;
    line.trim()
        .parse()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot parse input"))
}
