use std::ops::RangeInclusive;

use aoc_utils::*;
use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

fn part_1(input: &str) {
    // In this part, we want to count how many of the couple of ranges
    // have a complete overlap: either the first one completely
    // overlaps the second, or vice-versa.

    // Let's iterate over each couple of ranges.
    let result = input
        .lines()
        // We're gonna want to filter out all the couples that don't overlap.
        .filter(|l| {
            // This function parses a single range of the form
            // "start-end", as they are defined in the input scheme.
            let parse_range = |s: &str| {
                let (start, end) = s.split_once('-').unwrap();

                start.parse::<u32>().unwrap()..=end.parse::<u32>().unwrap()
            };

            // This function parses a couple of ranges of the form
            // "left,right", using the function above, as defined in
            // the input scheme.
            let parse_range_couple = |s: &str| {
                let (left, right) = s.split_once(',').unwrap();

                (parse_range(left), parse_range(right))
            };

            // This calculates whether the left range is completely included in the right range.
            let overlaps = |left: &RangeInclusive<u32>, right: &RangeInclusive<u32>| {
                left.start() >= right.start() && left.end() <= right.end()
            };

            // Parse our input into ranges.
            let (left, right) = parse_range_couple(l);

            // Since we want to check both cases (either left overlaps
            // right, or right overlaps left), we need to check both
            // cases.
            overlaps(&left, &right) || overlaps(&right, &left)
        })
        // The problem states we want to count the number of ranges that overlap.
        .count();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // In this part, we now count the number of ranges that overlap
    // even partially.

    let result = input
        .lines()
        .filter(|l| {
            let parse_range = |s: &str| {
                let (start, end) = s.split_once('-').unwrap();

                start.parse::<u32>().unwrap()..=end.parse::<u32>().unwrap()
            };

            let parse_range_couple = |s: &str| {
                let (left, right) = s.split_once(',').unwrap();

                (parse_range(left), parse_range(right))
            };

            let (left, right) = parse_range_couple(l);

            !(left.end() < right.start() || left.start() > right.end())
        })
        // The problem states we want to count the number of ranges that overlap.
        .count();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
