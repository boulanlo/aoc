use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"1000
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
    // The first part of the problem is basically summing the group of
    // numbers together, and finding the biggest one.

    let result = input
        .split("\n\n") // For every group of lines separated by empty lines...
        .map(|lines| {
            // ... iterate on each line ...
            lines
                .lines()
                .map(|line| {
                    // Convert the string into a number. Unwrapping is OK
                    // since the input is guaranteed to be valid.
                    line.parse::<u32>().unwrap()
                })
                .sum::<u32>() // ... and then sum the values together ...
        })
        .max() // ... and finally, find the biggest of these sums.
        // We can unwrap here because we're guaranteed to have at least
        // one number in our input.
        .unwrap();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // The second part is just an extension of the first part, like
    // most Advent of Code problems: instead of finding the biggest
    // sum, we want to find the 3 biggest, and then sum them together.
    //
    // We're gonna do mostly the same thing as the first part, only
    // differing in the end; this is because we still need to compute
    // the sum of each group.

    let result = input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        // Here, we're gonna use Itertools to create a sorted iterator
        // of our values. Two things of note:
        //
        // - This is O(n): this collects the values into a Vec, sorts
        // it, and re-creates the iterator. There is a better solution
        // in terms of algorithmic complexity, but on most machines, I
        // don't think it makes a noticeable difference, even with the
        // big input.
        //
        // - We can use the "unstable" version of the sort since we
        // don't care about the exact order of the groups, we just
        // care about the values themselves. This can speed up the
        // sort quite a bit.
        .sorted_unstable()
        // Since the sorted values are in increasing order, and we
        // want the 3 biggest, we need to start at the end, so let's
        // reverse the iterator. This should be O(1) given the smart
        // implementation of rev() on vector iterators.
        .rev()
        // Take the first 3 elements, which are the biggest as we
        // sorted the values beforehand.
        .take(3)
        // And finally, sum them.
        .sum::<u32>();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
