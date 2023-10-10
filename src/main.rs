use codeabbey::{sum, sum_loop};
use std::{collections::HashMap, io};

fn main() {
    let table = HashMap::from([
        (1, sum::run as fn(&str) -> String),
        (2, sum_loop::run),
    ]);

    println!("Problem: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let problem = input.trim().parse::<i32>().unwrap_or(table.len() as i32);
    let solver = table
        .get(&problem)
        .expect("Solution might not be implemented");

    println!("Input (Ctrl-D to end): ");
    let input = io::stdin().lines().map(|l| l.unwrap()).collect::<String>();
    let out = solver(input.trim());

    println!("Answer: {}", out);
}
