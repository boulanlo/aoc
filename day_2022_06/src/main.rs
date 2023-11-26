// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

// A helper function to convert a character into its respective index, a
// one-hot encoding from 0 to 25 (in bit index) for 'a' to 'z'.
fn char_to_bitmap(c: char) -> u32 {
    match c {
        // If the character is a lowercase ascii alphabetic character,
        // use its zero-indexed value in the alphabet as a shift left
        // amount.
        'a'..='z' => 1 << ((c as u8) - b'a'),
        // We are guaranteed our input only contains lowercase ascii
        // letters, so it's fine to panic here.
        _ => unreachable!(),
    }
}

// This function will find the index of the last character of the first
// non-repeating sequence of characters of a given length in a string.
fn first_nonrepeating_sequence_idx(input: &str, len: usize) -> usize {
    // Using our input...
    input
        // ...trim the whitespace around it... 
        .trim()
        // ...iterate on its characters...
        .chars()
        // ...convert the characters to their one-hot encodings...
        .map(char_to_bitmap)
        // ...and collect the resulting values into a vector. Note: we could 
        // theoretically do it in one go, but I want to use the `windows` 
        // function on slices to get arbitrary-length windows and it's only 
        // possible on a known-size collection.
        .collect::<Vec<u32>>()
        // Taking windows of the specified length...
        .windows(len)
        // ...bitwise OR the elements of each window to get a new iterator...
        .map(|array| array.iter().copied().reduce(|a, b| a | b).unwrap())
        // ...and find the position of the first character of a sequence that 
        // has the same amount of 1's as the requested length.
        .position(|bitmap| bitmap.count_ones() == len as u32)
        // We can unwrap here because we can assume we always have such 
        // sequence in our input.
        .unwrap()
        // And finally, add the length of the sequence to the index of the 
        // first character in order to find the last character of that sequence.
        // Note that in the prompt, characters are one-indexed, so we don't 
        // need to add a `- 1` which we would have had to do in the case of 
        // a zero-indexed answer.
        + len
}

fn part_1(input: &str) {
    // In this part, we need to find the index of the character that ends a
    // 4-character long sequence of non-repeating characters.

    let result = first_nonrepeating_sequence_idx(input, 4);

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // This part is the same, just with 14 characters long sequences.

    let result = first_nonrepeating_sequence_idx(input, 14);

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
