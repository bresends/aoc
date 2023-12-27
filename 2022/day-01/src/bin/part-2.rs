use day_01::process_part2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let result = process_part2(&input);
    println!("Result: {}", result);
}