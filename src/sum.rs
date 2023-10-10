/// Example:
/// 
/// input data:
/// `3 5`
/// 
/// answer:
/// `8`
pub fn run(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Unable to parse input"))
        .take(2)
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(run("2 2"), "4");
    }
}
