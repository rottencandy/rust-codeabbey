pub fn parse_input(input: &str) -> Result<(i32, i32), String> {
    let mut parts = input.split_whitespace().map(|s| s.parse::<i32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) => {
            Ok((a, b))
        }
        _ => {
            Err("Unable to parse!".into())
        }
    }
}
