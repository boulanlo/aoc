use std::collections::HashSet;

use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb
"#;

fn part_1(input: &str) {
    let res = input
        .chars()
        .tuple_windows()
        .position(|(a, b, c, d)| [a, b, c, d].into_iter().collect::<HashSet<char>>().len() == 4)
        .unwrap()
        + 4;

    println!("{res}");
}

fn part_2(input: &str) {
    let res = input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|a| a.iter().copied().collect::<HashSet<char>>().len() == 14)
        .unwrap()
        + 14;

    println!("{res}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
