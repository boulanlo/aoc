use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let number_of_stacks = (input.lines().last().unwrap().chars().count() + 1) / 4;

    let mut vecs = input
        .lines()
        .map(|l| {
            format!("{l} ")
                .chars()
                .chunks(4)
                .into_iter()
                .map(|mut c| {
                    if c.next() == Some('[') {
                        c.next()
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .fold(
            std::iter::repeat_with(Vec::new)
                .take(number_of_stacks)
                .collect::<Vec<_>>(),
            |mut acc, level| {
                for (v, c) in acc.iter_mut().zip(level.into_iter()) {
                    if let Some(c) = c {
                        v.push(c)
                    }
                }
                acc
            },
        );

    for v in &mut vecs {
        v.reverse()
    }

    vecs
}

fn parse_instructions(input: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    input.lines().map(|l| {
        let l = l
            .chars()
            .map(|c| if !c.is_ascii_digit() { ' ' } else { c })
            .collect::<String>();

        let mut i = l.split_ascii_whitespace();
        (
            i.next().unwrap().parse().unwrap(),
            i.next().unwrap().parse::<usize>().unwrap() - 1,
            i.next().unwrap().parse::<usize>().unwrap() - 1,
        )
    })
}

fn part_1(input: &str) {
    let mut input = iter_input_newline_separated(input);
    let stacks = input.next().unwrap();
    let instructions = input.next().unwrap();

    let mut stacks = parse_stacks(stacks);

    for (amount, src, dst) in parse_instructions(instructions) {
        let idx = stacks[src].len() - amount;
        let elems = stacks[src].drain(idx..).rev().collect::<Vec<_>>();
        stacks[dst].extend(elems);
    }

    for v in stacks {
        print!("{}", v.last().unwrap());
    }
    println!();
}

fn part_2(input: &str) {
    let mut input = iter_input_newline_separated(input);
    let stacks = input.next().unwrap();
    let instructions = input.next().unwrap();

    let mut stacks = parse_stacks(stacks);

    for (amount, src, dst) in parse_instructions(instructions) {
        let idx = stacks[src].len() - amount;
        let elems = stacks[src].drain(idx..).collect::<Vec<_>>();
        stacks[dst].extend(elems);
    }

    for v in stacks {
        print!("{}", v.last().unwrap());
    }
    println!();
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
