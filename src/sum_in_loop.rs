/// Input data will contain the total count of pairs to process in the first line.
/// The following lines will contain pairs themselves - one pair at each line.
/// Answer should contain the results separated by spaces.
///
/// Example:
///
/// data:
/// 3
/// 100 8
/// 15 245
/// 1945 54
///
/// answer:
/// 108 260 1999
pub fn run(input: &str) -> String {
    input
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("Unable to parse input"))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|vals| (vals[0] + vals[1]).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(run("3\n100 8\n15 245\n1945 54"), "108 260 1999");
    }
}
