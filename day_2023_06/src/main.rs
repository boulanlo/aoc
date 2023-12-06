use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

fn part_1(input: &str) {
    // In this part, we will want to find, for each race, how many ways we can
    // win them based on how long we press the RC boat's button.

    // First, parse the input. For each line in the input...
    let (times, distances) = input
        .lines()
        .map(|l| {
            // ...split it on the semicolon to remove the flavor text...
            l.split_once(':')
                .unwrap()
                .1
                .trim()
                // ...split the remaining string to get a list of numbers...
                .split_ascii_whitespace()
                // ...parse those numbers...
                .map(|v| v.parse::<u32>().unwrap())
                // ...and get a vector.
                .collect::<Vec<_>>()
        })
        // Now group everything in tuples...
        .tuples()
        // ...and get the first one. Thanks to type inference and the let
        // pattern, it knows we want a tuple of 2.
        .next()
        .unwrap();

    // Now the result: for each pair of time and distance (a race)...
    let result = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            // ...find the number of possibilities of winning.
            // We do this by iterating over all the possible button press
            // durations, calculating how far the boat would go, and only
            // keeping it if we get over the needed score.
            (0..=time).filter(|t| (time - t) * t > distance).count() as u32
        })
        // And finally, multiply these together.
        .product::<u32>();

    println!("{result}");
}

fn part_2(input: &str) {
    // For part 2, it's exactly the same thing, but we only have 1 race,
    // composed of the numbers of all the "previous" races.

    // Instead of parsing vectors of numbers, we parse 2 numbers.
    let (time, distance) = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .trim()
                // Only consider ASCII digit characters here and get a string
                // to parse as a number afterwards.
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .tuples()
        .next()
        .unwrap();

    // The result is calculated the same as one iteration of the previous
    // part's iterator.
    let result = (0..=time).filter(|t| (time - t) * t > distance).count();

    println!("{result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
