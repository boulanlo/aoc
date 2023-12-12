use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

// Computes the derivatives of all the lines of the input until we
// reach a derivative of 0.
fn derivatives(input: &str) -> impl Iterator<Item = Vec<Vec<i64>>> + '_ {
    // For each line of the input:
    input.lines().map(|l| {
        // Parse the line as a space-delimited list of signed numbers.
        let values = l
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        // We start with the base values.
        let mut derivatives = vec![values];

        // Then, in a loop:
        loop {
            // If the last derivative is all zeros, we can break out
            // of the loop.
            let last = derivatives.last().unwrap();
            if last.iter().all(|v| *v == 0) {
                break;
            } else {
                // If not, then we're not done. Take the last list of
                // derivatives, and compute a new one by finding the
                // difference between each couple of numbers.
                derivatives.push(last.iter().tuple_windows().map(|(a, b)| b - a).collect());
            }
        }

        // Now, we have all the derivatives for that line.
        derivatives
    })
}

// Extrapolates data for the input based on its derivatives. The
// function `f` here determines in which direction we extrapolate.
fn extrapolate<'a, F>(input: &'a str, f: F) -> impl Iterator<Item = i64> + '_
where
    F: Fn(i64, Vec<i64>) -> i64 + 'a + Copy,
{
    // Extrapolating is simply folding each list of derivatives
    // backwards using the function, which will go through each
    // successive list of derivative and give us a new number to apply
    // on the next derivatives.
    derivatives(input).map(move |derivatives| derivatives.into_iter().rev().fold(0, f))
}

fn part_1(input: &str) {
    // In this part, we extrapolate forwards: each successive number
    // is added to the last of the derivative.
    let result = extrapolate(input, |i, v| i + v.last().unwrap()).sum::<i64>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // In this part, we extrapolate backwards: each successive number
    // is subtracted from the first of the derivative.
    let result = extrapolate(input, |i, v| v.first().unwrap() - i).sum::<i64>();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
