pub fn run(input: &str) -> i32 {
    input
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().expect("Unable to parse input"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(run("3\n2 2 2"), 6);
    }
}
