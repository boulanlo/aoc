// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt for part 1.
const EXAMPLE_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

// Another example for part 2.
const EXAMPLE_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

fn part_1(input: &str) {
    // In this first part, we'll need to find the first and last digits in a
    // series of strings, concatenate them to get a two-digit number, and sum
    // these together to get the final result.

    // For each line in our input...
    let res = input
        .lines()
        .map(|l| {
            // ...compute all the digits in it...
            let digits = l
                .chars()
                .filter_map(|c| {
                    // ...and convert these characters to their numerical values.
                    if c.is_ascii_digit() {
                        Some((c as u8 - b'0') as u32)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // Concatenate the first and last digits into a single two-digit
            // number.
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        // And finally, sum these all together.
        .sum::<u32>();

    println!("Part 1: {res}");
}

// Given a string, this function returns an iterator over all the successive
// substrings ("abc" -> "a", "ab", "abc"), as well as the last character for
// each of them.
fn substrings_and_last_char(string: &str) -> impl Iterator<Item = (&str, char)> {
    // We are using byte indices and not character counts, so this will only
    // work on ASCII strings.
    assert!(string.is_ascii());

    (0..string.len()).map(|idx| {
        let s = string.split_at(idx + 1).0;
        (s, s.chars().last().unwrap())
    })
}

fn part_2(input: &str) {
    // In this part, we also need to consider that digits written in plain
    // english also count as digits.

    // Our list of string digits.
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // For each line of our input...
    let result = input
        .lines()
        .map(|l| {
            // ...build up the list of digits contained in it:
            let mut digits = Vec::new();

            // Go through all the accumulated substrings of the current line...
            for (s, c) in substrings_and_last_char(l) {
                // ...if the last character is an ASCII digit, add it to the
                // list...
                if c.is_ascii_digit() {
                    digits.push((c as u8 - b'0') as u32);
                // ...or if the substring ends with one of the written digits,
                // also add it to the list.
                } else if let Some(digit_idx) =
                    DIGITS.iter().position(|digit_str| s.ends_with(digit_str))
                {
                    digits.push(digit_idx as u32 + 1);
                }
            }

            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum::<u32>();

    println!("Part 2: {result}");
}

fn main() {
    part_1(EXAMPLE_1);
    part_2(EXAMPLE_2);
}
