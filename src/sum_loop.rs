pub fn run(input: &str) -> i32 {
/// Input data has the following format:
/// 
/// first line contains N - amount of values to sum;
/// second line contains N values themselves.
/// 
/// Answer should contain a single value - the sum of N values.
/// 
/// Example:
/// 
/// input data:
/// 8
/// 10 20 30 40 5 6 7 8
/// 
/// answer:
/// `126`
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
