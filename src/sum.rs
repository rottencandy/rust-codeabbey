use crate::parse::parse_two_numbers;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run(input: &str) -> i32 {
    let (a, b) = parse_two_numbers(input).expect("Expected two numbers");
    sum(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(run("2 2"), 4);
    }
}
