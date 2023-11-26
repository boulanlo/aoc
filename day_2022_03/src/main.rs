use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

fn part_1(input: &str) {
    // In this part, we're given a list of strings. In each string, we
    // must find the common character between the two halves of the
    // string. From this shared character, we calculate a value, and
    // sum them to find the final result.

    // Let's iterate over the lines of our input.
    let result = input
        .lines()
        .map(|line| {
            // Let's first define a function to convert a character to
            // its given value as specified in the prompt, *minus
            // one*. This difference is important to allow us to use
            // it for a bit array later.
            let char_value = |c: char| match c {
                // Characters from 'a' to 'z' get the values 0 to 25.
                'a'..='z' => (c as u8) - b'a',
                // Characters from 'A' to 'Z' get the values 26 to 51.
                'A'..='Z' => ((c as u8) - b'A') + 26,
                // We're guaranteed that our input doesn't have
                // non-ascii-alphabetic characters, so we can panic
                // here.
                _ => unreachable!(),
            };

            // Now, onto the meat of the solution. We're going to use
            // a 64-bit unsigned integer as a bit array to hold the
            // information of whether a character is present or not in
            // a string. We can do this because we have 52 distinct
            // characters: the lowercase and uppercase english
            // alphabets (both with 26 characters). If we convert each
            // character to a value using the previously defined
            // function, we can encode each character to a single bit
            // of our u64. Setting the bit to 1 will indicate that the
            // character is present, and 0 will indicate the character
            // is not present.
            //
            // This function will compute the bit array of character
            // presence for a given string.
            let compute_bit_array = |s: &str| {
                // We initialise the bit array to 0, aka "there are no
                // characters in the string".
                let mut bit_array: u64 = 0;

                // For each character in the string...
                for c in s.chars() {
                    // ... we set the corresponding bit to 1 using a
                    // bitwise OR operation.
                    bit_array |= 1 << char_value(c);
                }

                // At the end, we can return our bit array.
                bit_array
            };

            // We need to split the input line in two, to get the two
            // halves to compare.
            let (left, right) = line.split_at(line.len() / 2);

            // Now onto computing the common character between the two halves.
            (
                // First we need to compute the bit array of the left half.
                compute_bit_array(left)
		// We compute bit array of the right half, and then we logically AND
		// the two values. This will make sure that only the bits set to 1 in
		// both bit arrays will remain. We should only have one: the one
		// corresponding to the common character between the two.
		    & compute_bit_array(right)
            )
		// Right now, we have an integer where a 1 is set at
		// the nth bit, where n is the value of the common
		// character as given by the function "char_value"
		// defined above. To extract the position of this bit,
		// we just have to perform a base-2 logarithm.
		.ilog2()
		// Since the prompt specifies that character values
		// are one-indexed ('a' is 1, ...), we need to add 1
		// to our value.
                + 1
        })
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // The premise of the second part is largely the same, except that
    // instead of considering the two halves of each line, we need to
    // consider each line in groups of 3 lines. We can easily
    // generalise it to any group size.

    // Let's iterate over the lines of our input.
    let result = input
        .lines()
        // Group our lines in chunks of 3 lines.
        .chunks(3)
        .into_iter()
        .map(|group| {
            let char_value = |c: char| match c {
                'a'..='z' => (c as u8) - b'a',
                'A'..='Z' => ((c as u8) - b'A') + 26,
                _ => unreachable!(),
            };

            let compute_bit_array = |s: &str| {
                let mut bit_array: u64 = 0;

                for c in s.chars() {
                    bit_array |= 1 << char_value(c);
                }

                bit_array
            };

            // Let's iterate over the lines in our group...
            group
                // ... compute the bit array for that line ...
                .map(compute_bit_array)
                // ... and bitwise AND each bit array together using a
                // reducing operation.
                .reduce(|a, b| a & b)
                // We can unwrap here since we're guaranteed at least one group.
                .unwrap()
                .ilog2()
                + 1
        })
        .sum::<u32>();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
