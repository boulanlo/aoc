use std::{matches, ops::ControlFlow};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

enum Direction {
    Ascending,
    Descending,
}

fn is_tuple_safe(dir: Option<Direction>, (a, b): (u32, u32)) -> ControlFlow<(), Option<Direction>> {
    if matches!(dir, None | Some(Direction::Ascending)) && a < b && a.abs_diff(b) < 4 {
        ControlFlow::Continue(Some(Direction::Ascending))
    } else if matches!(dir, None | Some(Direction::Descending)) && a > b && a.abs_diff(b) < 4 {
        ControlFlow::Continue(Some(Direction::Descending))
    } else {
        ControlFlow::Break(())
    }
}

fn part_1(input: &str) {
    let result = input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .tuple_windows()
                .try_fold(None, is_tuple_safe)
                .is_continue()
        })
        .filter(|b| *b)
        .count();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let result = input
        .lines()
        .map(|s| {
            let line = s
                .split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let len = line.len();

            line.clone()
                .into_iter()
                .combinations(len - 1)
                .chain(std::iter::once(line))
                .any(|v| {
                    v.into_iter()
                        .tuple_windows()
                        .try_fold(None, is_tuple_safe)
                        .is_continue()
                })
        })
        .filter(|b| *b)
        .count();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
