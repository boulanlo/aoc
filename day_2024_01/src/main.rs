use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

fn part_1(input: &str) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|s| {
            let (l, r) = s.split_once("   ").unwrap();

            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    let result = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let (left, right) = input
        .lines()
        .map(|s| {
            let (l, r) = s.split_once("   ").unwrap();

            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .fold((Vec::new(), HashMap::new()), |(mut v, mut h), (l, r)| {
            v.push(l);
            h.entry(r).and_modify(|x| *x += 1).or_insert(1u32);
            (v, h)
        });

    let result = left
        .into_iter()
        .map(|x| x * right.get(&x).copied().unwrap_or_default())
        .sum::<u32>();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
