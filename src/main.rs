use advent_of_code_2022_rust::{day1, day2, day3};
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("=== DAY 1 ===");
    let contents = fs::read_to_string("input/day1.txt").unwrap();
    println!("{}", day1::part1(contents.lines()));
    println!("{}", day1::part2(contents.lines()));

    println!("=== DAY 2 ===");
    let contents = fs::read_to_string("input/day2.txt").unwrap();
    println!("{}", day2::part1(contents.lines()));
    println!("{}", day2::part2(contents.lines()));

    println!("=== DAY 3 ===");
    let contents = fs::read_to_string("input/day3.txt").unwrap();
    println!("{}", day3::part1(contents.lines()));
    println!("{}", day3::part2(contents.lines()));

    let elapsed = start.elapsed().as_millis();
    println!("Total execution time is {} milliseconds", elapsed);
}
