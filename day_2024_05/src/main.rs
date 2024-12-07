use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ops::ControlFlow,
    str::FromStr,
};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

struct Rules {
    inner: HashMap<u32, HashSet<u32>>,
}

impl FromStr for Rules {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rules {
            inner: s
                .lines()
                .map(|l| {
                    let (l, r) = l.split_once('|').unwrap();
                    (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
                })
                .fold(HashMap::new(), |mut h, (l, r)| {
                    h.entry(l)
                        .and_modify(|v| {
                            v.insert(r);
                        })
                        .or_insert_with(|| {
                            let mut h = HashSet::new();
                            h.insert(r);
                            h
                        });
                    h
                }),
        })
    }
}

impl Rules {
    fn middle_page_of_ordered<'a, I>(&'a self, i: I) -> impl Iterator<Item = u32> + '_
    where
        I: IntoIterator<Item = Vec<u32>> + 'a,
    {
        i.into_iter().filter_map(|v| {
            v.iter()
                .try_fold(HashSet::new(), |mut before, current| {
                    if self
                        .inner
                        .get(current)
                        .map(|rule| rule.intersection(&before).count() == 0)
                        .unwrap_or(true)
                    {
                        before.insert(*current);
                        ControlFlow::Continue(before)
                    } else {
                        ControlFlow::Break(())
                    }
                })
                .is_continue()
                .then_some(v[v.len() / 2])
        })
    }

    fn middle_page_of_unordered<'a, I>(&'a self, i: I) -> impl Iterator<Item = u32> + '_
    where
        I: IntoIterator<Item = Vec<u32>> + 'a,
    {
        i.into_iter().filter_map(|v| {
            let (_, ordered, error_found) = v.iter().fold(
                (HashSet::new(), Vec::new(), false),
                |(mut before, mut ordered, error_found), current| {
                    let incorrect_rules = self
                        .inner
                        .get(current)
                        .map(|rule| rule.intersection(&before).collect::<HashSet<_>>())
                        .unwrap_or_default();

                    if incorrect_rules.is_empty() {
                        before.insert(*current);
                        ordered.push(*current);
                        (before, ordered, error_found)
                    } else {
                        let smallest_idx = (0..ordered.len())
                            .find(|i| incorrect_rules.contains(&ordered[*i]))
                            .unwrap();

                        ordered.insert(smallest_idx, *current);
                        before.insert(*current);
                        (before, ordered, true)
                    }
                },
            );
            if error_found {
                Some(ordered[ordered.len() / 2])
            } else {
                None
            }
        })
    }
}

fn part_1(input: &str) {
    let (rules, input) = input.split_once("\n\n").unwrap();
    let rules = rules.parse::<Rules>().unwrap();

    let result = rules
        .middle_page_of_ordered(
            input
                .lines()
                .map(|l| l.split(',').map(|s| s.parse::<u32>().unwrap()).collect()),
        )
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let (rules, input) = input.split_once("\n\n").unwrap();
    let rules = rules.parse::<Rules>().unwrap();

    let result = rules
        .middle_page_of_unordered(
            input
                .lines()
                .map(|l| l.split(',').map(|s| s.parse::<u32>().unwrap()).collect()),
        )
        .sum::<u32>();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
