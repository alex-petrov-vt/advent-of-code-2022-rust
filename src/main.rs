use advent_of_code_2022_rust::day1;
use std::fs;

fn main() {
    let lines = fs::read_to_string("input/day1.txt").unwrap();

    println!("{}", day1::part1(&lines));
    println!("{}", day1::part2(&lines));
}
