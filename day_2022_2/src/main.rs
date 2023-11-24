use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"A Y
B X
C Z
"#;

fn part_1(input: &str) {
    let res = iter_input_raw(input)
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            let left = match left {
                "A" => 0u32,
                "B" => 1,
                "C" => 2,
                _ => unreachable!(),
            };

            let right = match right {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => unreachable!(),
            };

            (right + 1)
                + match right {
                    right if left == right => 3,
                    right if (left + 2) % 3 == right => 0,
                    _ => 6,
                }
        })
        .sum::<u32>();

    println!("{res}")
}

fn part_2(input: &str) {
    let res = iter_input_raw(input)
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            let left = match left {
                "A" => 0u32,
                "B" => 1,
                "C" => 2,
                _ => unreachable!(),
            };

            let right = match right {
                "X" => {
                    if left == 0 {
                        2
                    } else {
                        left - 1
                    }
                }
                "Y" => left,
                "Z" => (left + 1) % 3,
                _ => unreachable!(),
            };

            (right + 1)
                + match right {
                    right if left == right => 3,
                    right if (left + 2) % 3 == right => 0,
                    _ => 6,
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
