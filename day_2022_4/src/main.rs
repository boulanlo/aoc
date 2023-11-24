use aoc_utils::*;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

fn part_1(input: &str) {
    let res = iter_input_raw(input)
        .filter_map(|l| {
            let parse =
                |(a, b): (&str, &str)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());

            let (left, right) = l.split_once(',').unwrap();
            let (left_start, left_end) = parse(left.split_once('-').unwrap());
            let (right_start, right_end) = parse(right.split_once('-').unwrap());

            if (left_start >= right_start && left_end <= right_end)
                || (right_start >= left_start && right_end <= left_end)
            {
                Some(())
            } else {
                None
            }
        })
        .count();

    println!("{res}")
}

fn part_2(input: &str) {
    let res = iter_input_raw(input)
        .filter_map(|l| {
            let parse =
                |(a, b): (&str, &str)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());

            let (left, right) = l.split_once(',').unwrap();
            let (left_start, left_end) = parse(left.split_once('-').unwrap());
            let (right_start, right_end) = parse(right.split_once('-').unwrap());

            if left_end < right_start || left_start > right_end {
                None
            } else {
                Some(())
            }
        })
        .count();

    println!("{res}")
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
