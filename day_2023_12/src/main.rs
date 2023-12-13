use std::{collections::HashMap, convert::Infallible, str::FromStr};

// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Condition {
    Working,
    Defectuous,
    Unknown,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '#' => Condition::Working,
            '.' => Condition::Defectuous,
            '?' => Condition::Unknown,
            _ => unreachable!(),
        }
    }
}

impl From<bool> for Condition {
    fn from(value: bool) -> Self {
        if value {
            Condition::Working
        } else {
            Condition::Defectuous
        }
    }
}

#[derive(Clone)]
struct SpringLine {
    conditions: Vec<Condition>,
    expected_working: Vec<usize>,
}

impl SpringLine {
    fn possible_configurations(&self) -> usize {
        fn rec<'a>(
            conditions: &'a [Condition],
            expected_working: &'a [usize],
            memory: &mut HashMap<(&'a [Condition], &'a [usize], usize), usize>,
            current_group: usize,
        ) -> usize {
            if let Some(v) = memory.get(&(conditions, expected_working, current_group)) {
                *v
            } else {
                let v = match (conditions, expected_working) {
                    // Nothing left: OK
                    ([], []) => 1,
                    // One group left at the end: current group should be equal.
                    ([], [next_group]) => {
                        if *next_group == current_group {
                            1
                        } else {
                            0
                        }
                    }
                    // More than one group at the end: wrong.
                    ([], [_, _, ..]) => 0,
                    // A working spring + a group: current group + 1
                    // shouldn't be longer than this group, then
                    // continue with group + 1.
                    ([Condition::Working, c @ ..], [next_group, ..]) => {
                        if *next_group < current_group + 1 {
                            0
                        } else {
                            rec(c, expected_working, memory, current_group + 1)
                        }
                    }
                    // A working spring and no groups left: wrong.
                    ([Condition::Working, ..], []) => 0,
                    // A defectuous spring and a group: current group
                    // should either be 0 (then continue as 0), or
                    // equal to the current group (then continue as 0
                    // and using the rest of the groups). Otherwise
                    // it's wrong.
                    ([Condition::Defectuous, c @ ..], [next_group, e @ ..]) => {
                        if current_group == 0 {
                            rec(c, expected_working, memory, 0)
                        } else if current_group == *next_group {
                            rec(c, e, memory, 0)
                        } else {
                            0
                        }
                    }
                    // A defectuous spring and no group: current group must be 0.
                    ([Condition::Defectuous, c @ ..], []) => {
                        if current_group == 0 {
                            rec(c, expected_working, memory, 0)
                        } else {
                            0
                        }
                    }
                    // Unknown spring and next group: try both strategies.
                    ([Condition::Unknown, c @ ..], [next_group, e @ ..]) => {
                        let working = if *next_group < current_group + 1 {
                            0
                        } else {
                            rec(c, expected_working, memory, current_group + 1)
                        };

                        let defectuous = if current_group == 0 {
                            rec(c, expected_working, memory, 0)
                        } else if current_group == *next_group {
                            rec(c, e, memory, 0)
                        } else {
                            0
                        };

                        working + defectuous
                    }
                    // Unknown spring and no group: can't be working
                    // so only do as if it were defectuous.
                    ([Condition::Unknown, c @ ..], []) => {
                        if current_group == 0 {
                            rec(c, expected_working, memory, 0)
                        } else {
                            0
                        }
                    }
                };

                memory.insert((conditions, expected_working, current_group), v);
                v
            }
        }

        let mut memory = HashMap::new();

        rec(&self.conditions, &self.expected_working, &mut memory, 0)
    }
}

impl FromStr for SpringLine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (conditions, expected_working) = s.split_once(' ').unwrap();

        let conditions = conditions.chars().map(Into::into).collect();
        let expected_working = expected_working
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        Ok(SpringLine {
            conditions,
            expected_working,
        })
    }
}

fn part_1(input: &str) {
    let result = input
        .lines()
        .map(|l| {
            let line = l.parse::<SpringLine>().unwrap();

            line.possible_configurations()
        })
        .sum::<usize>();

    println!("{result}");
}

fn part_2(input: &str) {
    let result = input
        .lines()
        .map(|l| {
            let (conditions, expected_working) = l.split_once(' ').unwrap();

            let l = format!(
                "{} {}",
                (0..5)
                    .flat_map(|i| if i > 0 {
                        format!("?{conditions}")
                    } else {
                        conditions.to_string()
                    }
                    .chars()
                    .collect::<Vec<_>>())
                    .collect::<String>(),
                (0..5)
                    .map(|_| expected_working.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );

            let line = l.parse::<SpringLine>().unwrap();

            line.possible_configurations()
        })
        .sum::<usize>();

    println!("{result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
