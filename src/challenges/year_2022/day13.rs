use std::{cmp::Ordering, str::FromStr};

use color_eyre::{Report, Result};
use itertools::{EitherOrBoth, Itertools};

use crate::{runner::Messenger, Challenge, Dataset};

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListElement {
    Number(u32),
    List(Vec<ListElement>),
}

impl ListElement {
    pub fn from_chars<I>(iter: &mut I) -> Self
    where
        I: Iterator<Item = char>,
    {
        fn flush_accumulator(acc: &mut Vec<char>, v: &mut Vec<ListElement>) {
            if !acc.is_empty() {
                v.push(ListElement::Number(
                    acc.drain(..).collect::<String>().parse().unwrap(),
                ))
            }
        }

        let mut v = Vec::new();
        let mut number_acc = Vec::new();

        while let Some(c) = iter.next() {
            match c {
                ']' => {
                    flush_accumulator(&mut number_acc, &mut v);
                    break;
                }
                '[' => v.push(Self::from_chars(iter)),
                ',' => flush_accumulator(&mut number_acc, &mut v),
                c => number_acc.push(c),
            }
        }

        ListElement::List(v)
    }
}

impl FromStr for ListElement {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::from_chars(&mut s.chars()) {
            ListElement::Number(a) => Ok(ListElement::Number(a)),
            ListElement::List(mut l) => {
                debug_assert_eq!(l.len(), 1);
                Ok(l.pop().unwrap())
            }
        }
    }
}

impl PartialOrd for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn compare_lists(a: &[ListElement], b: &[ListElement]) -> Option<Ordering> {
            a.iter().zip_longest(b.iter()).find_map(|v| match v {
                EitherOrBoth::Both(x, y) => x.partial_cmp(y),
                EitherOrBoth::Left(_) => Some(Ordering::Greater),
                EitherOrBoth::Right(_) => Some(Ordering::Less),
            })
        }

        match (self, other) {
            (ListElement::Number(a), ListElement::Number(b)) => match a.cmp(b) {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Equal => None,
                Ordering::Greater => Some(Ordering::Greater),
            },
            (ListElement::Number(n), ListElement::List(l)) => {
                compare_lists(&[ListElement::Number(*n)], l)
            }
            (ListElement::List(l), ListElement::Number(n)) => {
                compare_lists(l, &[ListElement::Number(*n)])
            }
            (ListElement::List(a), ListElement::List(b)) => compare_lists(a, b),
        }
    }
}

impl Ord for ListElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug)]
struct DistressSignal {
    pairs: Vec<(ListElement, ListElement)>,
}

impl DistressSignal {
    pub fn sum_of_ordered_indices(&self) -> usize {
        self.pairs
            .iter()
            .enumerate()
            .filter_map(|(i, (a, b))| {
                a.partial_cmp(b).and_then(|res| {
                    if let Ordering::Less = res {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }

    pub fn find_decoder_keys(self) -> usize {
        let start = "[[2]]".parse::<ListElement>().unwrap();
        let end = "[[6]]".parse::<ListElement>().unwrap();

        std::iter::once_with(|| start.clone())
            .chain(std::iter::once_with(|| end.clone()))
            .chain(self.pairs.into_iter().flat_map(|(a, b)| vec![a, b]))
            .sorted()
            .enumerate()
            .filter_map(|(i, e)| {
                if e == start || e == end {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .product()
    }
}

impl<S> FromIterator<S> for DistressSignal
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let pairs = iter
            .into_iter()
            .filter_map(|s| {
                let s = s.as_ref();
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse().unwrap())
                }
            })
            .tuples()
            .collect();

        Self { pairs }
    }
}

pub struct Day13 {
    dataset: Dataset,
}

impl Day13 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day13 {
    fn name(&self) -> &'static str {
        "Distress Signal"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let signal = data.iter().collect::<DistressSignal>();

        Ok(signal.sum_of_ordered_indices().to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let signal = data.iter().collect::<DistressSignal>();

        Ok(signal.find_decoder_keys().to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
