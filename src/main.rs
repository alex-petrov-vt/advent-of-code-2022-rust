use advent_of_code_2022_rust::{day1, day2};
use std::fs;

fn main() {
    println!("=== DAY 1 ===");
    let contents = fs::read_to_string("input/day1.txt").unwrap();
    println!("{}", day1::part1(contents.lines()));
    println!("{}", day1::part2(contents.lines()));

    println!("=== DAY 2 ===");
    let contents = fs::read_to_string("input/day2.txt").unwrap();
    println!("{}", day2::part1(contents.lines()));
    println!("{}", day2::part2(contents.lines()));
}
