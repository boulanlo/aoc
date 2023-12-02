use std::collections::HashMap;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

// Let's define an enum to avoid storing arbitrary strings as the key of
// hashmaps. With it, non-matching colors will be caught at parse time and not
// later ("make invalid states unrepresentable").
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

// Parses a single grab of cubes into a HashMap where the key is the color.
fn parse_grab(grab: &str) -> HashMap<Color, u32> {
    // Trim unnecessary whitespace...
    grab.trim()
        // ...split on the commas (and space to avoid a trim later)...
        .split(", ")
        // ...and for each kind of color grabbed:
        .map(|pull| {
            // Split the number and the color;
            let (number, color) = pull.split_once(' ').unwrap();
            (
                // Parse the color and ensure we only have red, green or blue;
                match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => unreachable!(),
                },
                // And parse the number. We assume the number is also valid here
                // so unwrap is fine.
                number.parse::<u32>().unwrap(),
            )
        })
        // Collect it all in the map!!
        .collect::<HashMap<Color, u32>>()
}

fn part_1(input: &str) {
    // Part 1 essentially requires us to find all games where values of colors
    // grabbed don't go over certain thresholds.

    // For each line of the input...
    let result = input
        .lines()
        // ...and their game number (we don't need to parse it)...
        .enumerate()
        // ...only retain those that satisfy the problem condition:
        .filter_map(|(game, line)| {
            // For each grab into the bag...
            if line
                .split(|c| matches!(c, ':' | ';'))
                // (skip the first string here because it's "Game X:")
                .skip(1)
                // ...parse it into a hashmap of values...
                .map(parse_grab)
                // ...and ensure we never have more than 12 reds, 13 greens and
                // 14 blues.
                .all(|h| {
                    h.get(&Color::Red).copied().unwrap_or_default() <= 12
                        && h.get(&Color::Green).copied().unwrap_or_default() <= 13
                        && h.get(&Color::Blue).copied().unwrap_or_default() <= 14
                })
            {
                // If all grabs of the game satisfy the condition, then return
                // the number of the game (+1 here since we are 0-indexed and
                // the games are 1-indexed).
                Some(game + 1)
            } else {
                // Otherwise discard them.
                None
            }
        })
        // Finally, sum everything up.
        .sum::<usize>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // In part 2, we have to, instead, determine the minimum number of cubes of
    // each color that would make all grabs in a game valid. In other words,
    // for each game, for each color, it's the max value of that color across grabs.

    let result = input
        .lines()
        .map(|line| {
            line.split(|c| matches!(c, ':' | ';'))
                .skip(1)
                .map(parse_grab)
                // Instead of checking a condition and summing, we'll fold all our hashmaps into one.
                .fold(
                    // At the beginning, we have an empty hashmap.
                    HashMap::<Color, u32>::new(),
                    // Now, for each couple of hashmaps:
                    |mut a, b| {
                        // Go through every entry of the new one...
                        for (color, value) in b {
                            // ...and try to insert it into the old one:
                            a.entry(color)
                                // If it was already present, only keep the max.
                                .and_modify(|v| *v = (*v).max(value))
                                // If not, just insert the value.
                                .or_insert(value);
                        }
                        // Always keep the old one.
                        a
                    },
                )
                // Afterwards, we iterate on the values of that computed minimum
                // number of cubes...
                .values()
                // ...and multiply them all together.
                .product::<u32>()
        })
        // And finally, again, sum everything together to get the final value.
        .sum::<u32>();

    println!("Part 2: {result}")
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
