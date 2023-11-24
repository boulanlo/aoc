use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

fn part_1(input: &str) {
    println!(
        "{}",
        parse_input_newline_separated::<u32>(input)
            .map(|i| { i.sum::<u32>() })
            .sorted()
            .last()
            .unwrap()
    );
}

fn part_2(input: &str) {
    println!(
        "{}",
        parse_input_newline_separated::<u32>(input)
            .map(|i| { i.sum::<u32>() })
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>()
    );
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
