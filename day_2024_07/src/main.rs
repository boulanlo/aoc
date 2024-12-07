use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn concatenate(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse().unwrap()
}

type Operation = Box<dyn Fn(u64, u64) -> u64>;

impl FromStr for Equation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (res, op) = s.split_once(": ").unwrap();

        let result = res.parse().unwrap();

        let operands = op.split(' ').map(|s| s.parse().unwrap()).collect();

        Ok(Self { result, operands })
    }
}

impl Equation {
    fn is_solvable(&self, operators: &[Operation]) -> Option<u64> {
        itertools::repeat_n(operators.iter(), self.operands.len() - 1)
            .multi_cartesian_product()
            .find_map(|mut ops| {
                if self
                    .operands
                    .iter()
                    .copied()
                    .reduce(|acc, e| ops.pop().unwrap()(acc, e))
                    == Some(self.result)
                {
                    Some(self.result)
                } else {
                    None
                }
            })
    }
}

fn part_1(input: &str) {
    let equations: Vec<Equation> = input.lines().map(|l| l.parse().unwrap()).collect();

    let result = equations
        .iter()
        .filter_map(|e| {
            e.is_solvable(&[Box::new(add) as Operation, Box::new(multiply) as Operation])
        })
        .sum::<u64>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let equations: Vec<Equation> = input.lines().map(|l| l.parse().unwrap()).collect();

    let result = equations
        .iter()
        .filter_map(|e| {
            e.is_solvable(&[
                Box::new(add) as Operation,
                Box::new(multiply) as Operation,
                Box::new(concatenate) as Operation,
            ])
        })
        .sum::<u64>();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
