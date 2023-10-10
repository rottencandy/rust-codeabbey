pub fn run(input: &str) -> i32 {
/// Example:
/// 
/// input data:
/// `3 5`
/// 
/// answer:
/// `8`
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Unable to parse input"))
        .take(2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(run("2 2"), 4);
    }
}
