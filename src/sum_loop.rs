pub fn run(input: &str) -> i32 {
    input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(run("2 2 2"), 6);
    }
}
