use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

// We can model our stacks as a vector of stacks of characters; a
// stack is also just a vector too.
type Stacks = Vec<Vec<char>>;

// Let's parse the first half of the input to get the initial state of
// the stacks.
fn parse_stacks(input: &str) -> Stacks {
    // First, let's get the number of stacks. We can easily compute it
    // by realising that each stack will be represented by 4
    // characters: eithe whitespace for nothing, or '[', a letter, ']'
    // and ' '. Since the last stack is missing the last space, we add
    // it manually.
    let number_of_stacks = (input.lines().next().unwrap().chars().count() + 1) / 4;

    // Let's iterate on each line of the stack.
    input
        .lines()
        // Go from the bottom, since we're working on bottom-to-top stacks.
        .rev()
        // Skip the first line here (so the last line of the input),
        // which contains the labels of stacks, which we don't need.
        .skip(1)
        // And for each of these lines...
        .map(|line| {
            // Get an iterator on the characters of the line
            line.chars()
                // Add a space at the end
                .chain(std::iter::once(' '))
                // Divide the line in chunks of 4 characters
                .chunks(4)
                .into_iter()
                // For each of those stack slices...
                .map(|mut stack| {
                    // If it's an element to put in the stack, get the
                    // letter associated to it. Otherwise, return a
                    // None.
                    if let Some('[') = stack.next() {
                        Some(stack.next().unwrap())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        // Now we construct the vector of stacks from the stack slices.
        .fold(
            // At first, we have a vec of empty stacks.
            vec![Vec::new(); number_of_stacks],
            |mut acc, line| {
                // For each potential element in each stack...
                for (stack, elem) in acc.iter_mut().zip(line.into_iter()) {
                    // If there is an element, add it to the stack.
                    if let Some(elem) = elem {
                        stack.push(elem);
                    }
                }
                acc
            },
        )
}

fn part_1(input: &str) {
    // The goal here is to parse the stacks given in the input and
    // perform "crane-like" moves of elements between them, taking
    // elements one at a time.

    // Let's separate our input into two sections: the initial stacks
    // state and the move instructions. They are separated by a double
    // newline sequence in the input.
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    // We parse our stacks based on the function defined above.
    let mut stacks = parse_stacks(stacks);

    // For every move instruction...
    for line in moves.lines() {
        // We want to extract the amount, source and destination
        // stacks from the line:
        let [amount, source, dest] = line
            // Split every word in the string
            .split_ascii_whitespace()
            // Keep only those that we can convert to numbers
            .filter_map(|token| token.parse::<usize>().ok())
            // Collect into a vector
            .collect::<Vec<_>>()
            // We can convert it into a sized array and unwrap because
            // we know we're always going to get 3 numbers.
            .try_into()
            .unwrap();

        // Now we're going to perform the move. First, get the index
        // of the first element to be removed in the source stack.
        let start_idx = stacks[source - 1].len() - amount;

        // Then, extract those elements:
        let crates = stacks[source - 1]
            // Drain the vector starting at the calculated index up to the end...
            .drain(start_idx..)
            // Reverse the order since we are going to simulate picking up one element at a time...
            .rev()
            // Collect it into a temporary vector.
            .collect::<Vec<_>>();

        // Finally, extend the destination stack with the extracted
        // elements.
        stacks[dest - 1].extend(crates);
    }

    print!("Part 1: ");
    // Now that we have moved everything, we need to print the topmost
    // elements of each stack. For each stack in our list of stacks...
    for stack in stacks {
        // Print the last element. We know there is always going to be
        // at least one element in each stack, so unwrapping is
        // fine. Note that we use print!() and not println!() so that
        // we can print every character on the same line.
        print!("{}", stack.last().unwrap())
    }
    // Print a newline at the end to end the result string.
    println!();
}

fn part_2(input: &str) {
    // The part 2 is going to be identical to part 1, but we are only
    // going to remove the reverse order on the extracted elements
    // from a stack, since we can now pick up all the elements in one
    // go.

    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);

    for line in moves.lines() {
        let [amount, source, dest] = line
            .split_ascii_whitespace()
            .filter_map(|token| token.parse::<usize>().ok())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let start_idx = stacks[source - 1].len() - amount;

        let crates = stacks[source - 1]
            .drain(start_idx..)
            // We remove the reverse order from here.
            .collect::<Vec<_>>();

        stacks[dest - 1].extend(crates);
    }

    print!("Part 2: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
    println!();
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
