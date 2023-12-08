use std::collections::HashMap;

use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The examples given in the prompt.
const _EXAMPLE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

const EXAMPLE_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

const EXAMPLE_3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

// The two directions we can take when following a map.
enum Direction {
    Left,
    Right,
}

// We can convert the characters from the input into directions.
impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

// Parses the input string into a list of directions to take, and the
// maps to follow.
fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    // Separate the directions from the maps.
    let (directions, maps) = input.split_once("\n\n").unwrap();

    // The directions are the different 'L' and 'R' characters, parsed
    // into our enum.
    let directions = directions
        .trim()
        .chars()
        .map(Direction::from)
        .collect::<Vec<_>>();

    // The maps are aggregated into a hashmap, where the key is the
    // name of the location, and the values are the two choices you
    // can make at that location.
    let maps = maps
        .lines()
        .map(|l| {
            let (name, choices) = l.split_once(" = ").unwrap();
            let (left, right) = choices
                .split(|c: char| !c.is_ascii_alphanumeric())
                .filter(|s| !s.is_empty())
                .tuples()
                .next()
                .unwrap();

            (name, (left, right))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    (directions, maps)
}

// Computes the amount of steps it takes to go from the start to the end.
fn compute_path<E>(
    // The cyclic list of directions to follow.
    directions: &[Direction],
    // The maps to navigate.
    maps: &HashMap<&str, (&str, &str)>,
    // The starting node.
    start: &str,
    // A function that determines if a given node is an end node.
    end: E,
) -> usize
where
    E: Fn(&str) -> bool,
{
    // First, create the cyclic iterator over the directions.
    let mut directions = directions.iter().cycle();

    // We are going to build the number of steps to take through the
    // number of elements of this iterator.
    std::iter::successors(
        // At first, we start at the starting node of the map, and we
        // get two choices.
        maps.get(start),
        // Now, we need to decide what to do at each step.
        |(left, right)| {
            // First, make our choice by finding out where to go next,
            // left or right. We can unwrap the direction here since
            // it's a cyclic iterator.
            let choice = match directions.next().unwrap() {
                Direction::Left => left,
                Direction::Right => right,
            };

            // If the choice we end up making is considered the end,
            // we can end our loop here. Otherwise, continue using the
            // choice we made.
            if end(choice) {
                None
            } else {
                maps.get(choice)
            }
        },
    )
    // Finally, we count the amount of steps performed.
    .count()
}

// Computes the greatest common divisor of two numbers, using the
// binary GCD algorithm.
fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut d = 0;

    loop {
        if a == b {
            return a * 2usize.pow(d);
        } else {
            match (a % 2, b % 2) {
                (0, 0) => {
                    a /= 2;
                    b /= 2;
                    d += 1;
                }
                (0, _) => {
                    a /= 2;
                }
                (_, 0) => {
                    b /= 2;
                }
                (_, _) => {
                    if a < b {
                        std::mem::swap(&mut a, &mut b);
                    }
                    let c = a - b;
                    a = c / 2;
                }
            }
        }
    }
}

// Computes the least common multiple of two numbers, using their GCD.
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part_1(input: &str) {
    // In part 1, we need to find how many steps there are between
    // node AAA and node ZZZ.

    // We first parse our input.
    let (directions, maps) = parse_input(input);

    // The result of this part is the amount of steps from AAA to ZZZ.
    let result = compute_path(&directions, &maps, "AAA", |s| s == "ZZZ");

    println!("{result}");
}

fn part_2(input: &str) {
    // In this part, we need to start from every node ending in A, and
    // go up until a node that ends with Z. The expected result is the
    // amount of steps it takes for ALL paths starting with nodes that
    // end with A to converge at the same time to a node that ends
    // with Z.

    let (directions, maps) = parse_input(input);

    // For each node in the map...
    let result = maps
        .keys()
        // Only consider our starting nodes.
        .filter(|s| s.ends_with('A'))
        // For each starting node, compute the number of steps it
        // takes to reach one that ends with Z.
        .map(|start| compute_path(&directions, &maps, start, |s| s.ends_with('Z')))
        // To find out how many steps it takes for all the paths to
        // converge simultaneously to an end node, we just need to
        // find out the least common multiple of all of the lengths of
        // the paths.
        .reduce(lcm)
        .unwrap();

    println!("{result}");
}

fn main() {
    let input = EXAMPLE_2;
    part_1(input);
    part_2(EXAMPLE_3);
}
