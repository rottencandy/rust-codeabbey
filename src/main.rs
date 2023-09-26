use codeabbey::sum;
use std::{collections::HashMap, io};

fn main() {
    let table = HashMap::from([(1, sum::run)]);
    let mut input = String::new();

    println!("Problem: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let problem = input.trim().parse::<i32>().unwrap_or(table.len() as i32);
    let solver = table
        .get(&problem)
        .expect("Solution might not be implemented");

    println!("Input: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let out = solver(&input.trim());

    println!("Answer: {}", out);
}
