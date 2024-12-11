use std::{collections::HashMap, convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"125 17"#;

fn split(mut n: u64) -> (u64, u64) {
    let half = (n.ilog10() + 1) / 2;

    let mut right = 0;
    for i in 0..half {
        right += (n % 10) * 10u64.pow(i);
        n /= 10;
    }

    (n, right)
}

struct Field {
    stones: HashMap<u64, usize>,
}

impl FromStr for Field {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            stones: s
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .fold(HashMap::new(), |mut h, v| {
                    h.entry(v).and_modify(|v| *v += 1).or_insert(1);
                    h
                }),
        })
    }
}

impl Field {
    fn blink(&self, times: usize) -> usize {
        let mut memory: HashMap<u64, (u64, Option<u64>)> = HashMap::new();

        std::iter::successors(Some(self.stones.clone()), |stones| {
            Some(
                stones
                    .iter()
                    .map(|(&stone, &amount)| {
                        let result = memory.get(&stone).cloned().unwrap_or_else(|| {
                            let result = if stone == 0 {
                                (1, None)
                            } else if stone.ilog10() % 2 == 1 {
                                let (left, right) = split(stone);
                                (left, Some(right))
                            } else {
                                (stone * 2024, None)
                            };

                            memory.insert(stone, result);
                            result
                        });

                        (amount, result)
                    })
                    .fold(HashMap::new(), |mut h, (amount, (left, right))| {
                        h.entry(left).and_modify(|v| *v += amount).or_insert(amount);
                        if let Some(right) = right {
                            h.entry(right)
                                .and_modify(|v| *v += amount)
                                .or_insert(amount);
                        }

                        h
                    }),
            )
        })
        .nth(times)
        .unwrap()
        .values()
        .sum()
    }
}

fn part_1(input: &str) {
    let field: Field = input.parse().unwrap();

    let result = field.blink(25);

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let field: Field = input.parse().unwrap();

    let result = field.blink(75);

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
