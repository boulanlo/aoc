use std::collections::HashMap;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

fn part_1(input: &str) {
    // The first part asks us to find all numbers adjacent (even diagonally) to
    // a non-dot symbol and sum them.

    // First, let's extract the symbols into a 2D array: each row is a vector of
    // X coordinates of the symbols. For each line in our input...
    let symbols = input
        .lines()
        .map(|l| {
            // ...go through each character...
            l.char_indices()
                // ...and only keep the indices of non-dots, non-number ones.
                .filter_map(|(i, c)| {
                    if !c.is_ascii_digit() && c != '.' {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Now, onto the actual calculation. For each line of our input, again...
    let result = input
        .lines()
        // ...enumerate the lines to get the y coordinate...
        .enumerate()
        // ...and aggregate every number that satisfies our conditions:
        .flat_map(|(y, line)| {
            // First, our manual iterator over the characters of the line and
            // their indices (aka X coordinates).
            let mut chars = line.char_indices().peekable();

            // And also a vector for all the numbers that are adjacent to a
            // symbol.
            let mut matches = Vec::new();

            // Now, going through our characters...
            while let Some((x, c)) = chars.next() {
                // ...and skipping over non-number ones...
                if c.is_ascii_digit() {
                    // Begin collecting all the subsequent numbers into a string.
                    let mut s = c.to_string();
                    while let Some((_, '0'..='9')) = chars.peek() {
                        s.push(chars.next().unwrap().1);
                    }

                    // Define our search area. First in the X coordinates, it's
                    // the entire width of the number, with a margin of 1. Don't
                    // forget to take into account the boundaries.
                    let start_x = if x == 0 { 0 } else { x - 1 };
                    let end_x = if x == 0 {
                        4
                    } else {
                        (x + s.len() + 1).min(line.chars().count())
                    };

                    // Next in the Y coordinates, it's one above, one below and
                    // the current. Same as before, take into account boundaries.
                    let start_y = y.saturating_sub(1);
                    let end_y = (y + 1).min(symbols.len() - 1);

                    // For every list of symbol in the Y range...
                    if symbols[start_y..=end_y]
                        .iter()
                        // ...if any of them is adjacent to the number...
                        .any(|v| (start_x..end_x).any(|x| v.contains(&x)))
                    {
                        // ...add it to our match list and break from the loop
                        // (`any` is short-circuiting).
                        matches.push(s.parse::<u32>().unwrap());
                    }
                }
            }

            // Return all the matches we found.
            matches
        })
        // And finally, sum everything.
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // Now for the second part, only consider gears ('*' symbols). The goal is
    // the product of all the numbers adjacent to each gear, but only if there
    // are at least two numbers.

    // Let's compute the positions of the gears in advance. Same as with symbols,
    // just a simpler condition.
    let gears = input
        .lines()
        .map(|l| {
            l.char_indices()
                .filter_map(|(i, c)| if c == '*' { Some(i) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Now for the result, same as the previous part, go through the lines of
    // the input...
    let result = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut chars = line.char_indices().peekable();

            // We'll use a hashmap this time, in order to aggregate gear-number
            // associations across lines.
            let mut matches = HashMap::<(usize, usize), Vec<u32>>::new();

            while let Some((x, c)) = chars.next() {
                if c.is_ascii_digit() {
                    let mut s = c.to_string();
                    while let Some((_, '0'..='9')) = chars.peek() {
                        s.push(chars.next().unwrap().1);
                    }
                    let start = if x == 0 { 0 } else { x - 1 };
                    let end = if x == 0 {
                        4
                    } else {
                        (x + s.len() + 1).min(line.chars().count())
                    };
                    let start_y = y.saturating_sub(1);
                    let end_y = (y + 1).min(gears.len() - 1);

                    // Pre-compute the value to add in advance.
                    let val = s.parse::<u32>().unwrap();

                    // We're going to find every gear the number is attached to.
                    for (key, value) in
                        // For each row of gears in the Y range...
                        gears[start_y..=end_y]
                            .iter()
                            // ...get the row's relative Y coordinate...
                            .enumerate()
                            // ...and find every gear adjacent to our number:
                            .flat_map(|(dy, v)| {
                                // Compute the absolute Y coordinate of the
                                // current row.
                                let gear_y = start_y + dy;

                                // For each gear in the row, only keep the ones
                                // whose X coordinates fall into the correct
                                // adjacency range.
                                v.iter().copied().filter_map(move |gear_x| {
                                    if (start..end).contains(&gear_x) {
                                        Some(((gear_x, gear_y), val))
                                    } else {
                                        None
                                    }
                                })
                            })
                    {
                        // Either add the gear and the current number to the
                        // matches if it wasn't present before, or add the
                        // number to the gear's list if it was.
                        matches
                            .entry(key)
                            .and_modify(|v| v.push(value))
                            .or_insert(vec![value]);
                    }
                }
            }

            matches
        })
        // Now, reduce all the hashmaps into one, by merging the lists of
        // matching gear coordinates.
        .fold(HashMap::new(), |mut a, b| {
            for (key, value) in b {
                a.entry(key)
                    .and_modify(|v: &mut Vec<u32>| v.extend(value.iter().copied()))
                    .or_insert(value);
            }
            a
        })
        // Iterate over the list of numbers of each gear...
        .into_values()
        // ...only keep those with more than 1 number and compute the product of
        // those values...
        .filter_map(|v| {
            if v.len() > 1 {
                Some(v.into_iter().product::<u32>())
            } else {
                None
            }
        })
        // ...and finally, sum everything.
        .sum::<u32>();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
