use std::collections::HashSet;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

// Returns the number of matching numbers of each scratch card in the input.
fn matching_numbers(input: &str) -> impl Iterator<Item = u32> + '_ {
    // For each line in the input...
    input.lines().map(|line| {
        // Remove the "Card X:" bit.
        let (_, line) = line.split_once(':').unwrap();
        // Separate the winning numbers and the test numbers.
        let (winning, current) = line.split_once('|').unwrap();

        // Parse and collect both set of numbers into hash sets. We can assume
        // that the numbers we get in the input are well-formed, so unwrapping
        // the `parse()` is not a big deal.
        let winning = winning
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let current = current
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        // Count the number of matching numbers by using the intersection
        // between the two sets.
        winning.intersection(&current).count() as u32
    })
}

fn part_1(input: &str) {
    // In part 1, we just count the number of matching numbers and calculate a
    // score, that is literally just 2^(matching numbers - 1), and sum those
    // scores up.

    // For each set of matching numbers in each card...
    let result = matching_numbers(input)
        // ...only keep those that have at least 1 matching number, and
        // calculate the score...
        .filter_map(|matching| matching.checked_sub(1).map(|v| 2u32.pow(v)))
        // ...and sum everything up.
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // In part 2 this time, we duplicate cards whenever we get matching numbers.

    // For each set of cards and their matching numbers...
    let result = matching_numbers(input)
        // ...enumerated to get the index of each card...
        .enumerate()
        // ...compute the number of cards we obtain:
        .fold(
            // At first, we collect each card.
            input.lines().map(|_| 1).collect::<Vec<u32>>(),
            // And then, for each subsequent card and the number of matching
            // numbers:
            |mut copies, (i, matching)| {
                // Update the number of copies of the X subsequent cards by the
                // number of copies of this current card, where X is the count
                // of matching numbers of the current card.
                for dx in 0..matching {
                    copies[i + (dx as usize) + 1] += copies[i];
                }

                copies
            },
        )
        // And finally, we sum the amount of copies of each card to
        // get the result.
        .into_iter()
        .sum::<u32>();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
