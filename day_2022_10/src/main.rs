use std::fmt;

use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt. I put it in its own file since it's so tall.
const EXAMPLE: &str = include_str!("example.txt");

// This function will parse the program passed as a parameter and return an
// iterator over the values of the X register over the course of each cycle.
fn program_to_register_values(input: &str) -> impl Iterator<Item = i32> + '_ {
    // We will need to store each delta performed by an `addx`, since they take
    // effect on the cycle *after* the instruction.
    let mut next_delta = 0;

    // Let's construct an iterator over the different deltas (values added or
    // subtracted) of register X.
    //
    // For each line of the program...
    let mut delta_iterator = input
        .lines()
        // ...transform the line into one or more values:
        .flat_map(move |line| {
            // First, create an iterator over the tokens of the line.
            let mut tokens = line.split_ascii_whitespace();

            // What is the first token (i.e. what is the operation)?
            match tokens.next() {
                // A `noop`: only one cycle.
                Some("noop") => {
                    // The delta is whatever the delta was set to before us.
                    let v = vec![next_delta];
                    // A `noop` never modifies register X, so the delta is now 0.
                    next_delta = 0;
                    v
                }
                // An `addx`: two cycles.
                Some("addx") => {
                    // On the first cycle, the previous delta takes effect. On
                    // the second cycle, nothing happens.
                    let v = vec![next_delta, 0];
                    // The next delta is the operand of the `addx` instruction.
                    next_delta = tokens.next().unwrap().parse::<i32>().unwrap();
                    v
                }
                // We are guaranteed a valid input, so we can panic if there is
                // no instruction or if it's an unknown one.
                _ => unreachable!(),
            }
        });

    // And now, we need to construct the final iterator, that will yield for
    // each cycle the true value of register X, and not merely the changes
    // applied to it.
    std::iter::successors(
        // We first start with a value of 1, as specified in the prompt.
        Some(1),
        // Now, given the previous value of X, the current value is the previous
        // plus the delta at this cycle. If there are no more cycles (the
        // program ended), we can return None.
        move |x| delta_iterator.next().map(|dx| *x + dx),
    )
}

fn part_1(input: &str) {
    // On the first part, we need to calculate the sum of "signal strengths":
    // the signal strength is, at any cycle, the number of the cycle times the
    // value of register X. We only want the signal strengths of cycle 20, and
    // the one every 40 cycles after that (so 20, 60, 100, ...).

    // For each successive value of the register X at each cycle...
    let result = program_to_register_values(input)
        // ...enumerate the cycles...
        .enumerate()
        // ...skip the first 20 cycles...
        .skip(20)
        // ...and step by 40 cycles...
        .step_by(40)
        // ...so that for each couple (cycle number, register X value), we
        // multiply them...
        .map(|(cycle, x)| cycle as i32 * x)
        // ...and sum the product.
        .sum::<i32>();

    println!("Part 1: {result}");
}

/// An image on the CRT monitor of the problem.
struct Image {
    /// The pixels of the CRT, stored as bits in 64-bit integers. There are
    /// only 40 pixels in width, so the 24 most significant bits of each integer
    /// is unused.
    pixels: [u64; 6],
}

impl Image {
    /// Construct an Image by running the program passed and interpreting the X
    /// register as the position for a sprite to display.
    pub fn new(program: &str) -> Self {
        // Let's construct our pixel array.
        //
        // For each of successive values of register X..
        let pixels = program_to_register_values(program)
            // ...enumerate the cycles...
            .enumerate()
            // ...skip the first one as it's a side-product of the way the
            // iterator is constructed...
            .skip(1)
            // ...and divide these cycles into chunks of 40.
            .chunks(40)
            // Now, for each of those chunks of cycles and register X values:
            .into_iter()
            .map(|line| {
                // Create a blank pixel line.
                let mut pixel_line = 0u64;

                // For each cycle number and associated register X value:
                for (cycle, x) in line {
                    // If the cycle number (modulo 40 as we wrap around) is
                    // interpreted as a horizontal position and is contained by
                    // the 3-pixel-wide sprite (determined by the value of the
                    // X register):
                    if (x - 1..=x + 1).contains(&(((cycle - 1) % 40) as i32)) {
                        // Set the corresponding bit of the pixel line.
                        pixel_line |= 1 << ((cycle - 1) % 40)
                    }
                }

                pixel_line
            })
            // Collect those pixel lines into a vector.
            .collect::<Vec<_>>()
            // The input is guaranteed to execute to exactly 240 cycles (6 rows
            // of 40 pixels), so we can use `try_into()` to convert the
            // dynamically-sized Vec we just constructed into a statically-sized
            // array, and unwrap.
            .try_into()
            .unwrap();

        Self { pixels }
    }
}

// Now onto displaying the image.
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // For each line on the monitor...
        for line in self.pixels {
            // Write to the terminal the following string:
            writeln!(
                f,
                "{}",
                // The integer, formatted to show the 40 least-significant bits
                // in binary...
                format!("{line:040b}")
                    // ...with zeros replaced by dots...
                    .replace('0', ".")
                    // ...and ones replaced by hashes.
                    .replace('1', "#")
                    // Now to put it in the correct orientation, take the
                    // characters of this string...
                    .chars()
                    // ...reverse the iterator...
                    .rev()
                    // ...and collect it to a new string.
                    .collect::<String>()
            )?;
        }

        Ok(())
    }
}

fn part_2(input: &str) {
    // The second part requires us to interpret the value of the X register
    // throughout the program's execution as the horizontal position of a
    // 3-pixel-wide sprite, and to draw it on a 40x6 screen based on the cycle
    // numbers.
    let image = Image::new(input);

    println!("Part 2:\n{image}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
