use std::collections::HashSet;

use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

fn part_1(input: &str) {
    let res = iter_input_raw(input)
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);

            match left
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&right.chars().collect())
                .next()
                .unwrap()
            {
                c @ 'a'..='z' => (*c as u32) - 96,
                c @ 'A'..='Z' => (*c as u32) - 38,
                _ => unreachable!(),
            }
        })
        .sum::<u32>();

    println!("{res}")
}

fn part_2(input: &str) {
    let res = iter_input_raw(input)
        .chunks(3)
        .into_iter()
        .map(|l| {
            match l
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold(None::<HashSet<char>>, |mut acc, h| {
                    if let Some(acc) = acc.as_mut() {
                        *acc = acc.intersection(&h).copied().collect()
                    } else {
                        acc = Some(h)
                    }
                    acc
                })
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
            {
                c @ 'a'..='z' => (c as u32) - 96,
                c @ 'A'..='Z' => (c as u32) - 38,
                _ => unreachable!(),
            }
        })
        .sum::<u32>();

    println!("{res}")
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
