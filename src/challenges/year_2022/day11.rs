use std::{cell::Cell, str::FromStr};

use color_eyre::{Report, Result};
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

type MonkeyId = usize;
type Item = u64;
type Level = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperationKind {
    Multiplication,
    Addition,
}

impl FromStr for OperationKind {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(OperationKind::Addition),
            "*" => Ok(OperationKind::Multiplication),
            _ => Err(color_eyre::eyre::eyre!("Unknown operation kind '{s}'")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Immediate(Item),
    Old,
}

impl Operand {
    pub fn resolve(&self, item: Item) -> Item {
        match self {
            Operand::Immediate(v) => *v,
            Operand::Old => item as Item,
        }
    }
}

impl FromStr for Operand {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            n => Ok(Operand::Immediate(n.parse()?)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Operation {
    left: Operand,
    kind: OperationKind,
    right: Operand,
}

impl Operation {
    pub fn apply_on(&self, item: Item) -> Item {
        let left = self.left.resolve(item);
        let right = self.right.resolve(item);

        match self.kind {
            OperationKind::Multiplication => left * right,
            OperationKind::Addition => left + right,
        }
    }
}

impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn assert_str<'a, I>(s: &mut I, cmp: &str) -> Result<(), Report>
        where
            I: Iterator<Item = &'a str>,
        {
            s.next()
                .ok_or_else(|| color_eyre::eyre::eyre!("Invalid operation format"))
                .and_then(|s| {
                    if s == cmp {
                        Ok(())
                    } else {
                        Err(color_eyre::eyre::eyre!(
                            "Expected '{cmp}' in operation format, got '{s}'"
                        ))
                    }
                })?;
            Ok(())
        }

        fn get<'a, I, T>(s: &mut I) -> Result<T, Report>
        where
            I: Iterator<Item = &'a str>,
            T: FromStr<Err = Report>,
        {
            s.next()
                .ok_or_else(|| color_eyre::eyre::eyre!("Invalid operation format"))
                .and_then(FromStr::from_str)
        }

        let mut s = s.split_ascii_whitespace();
        assert_str(&mut s, "new")?;
        assert_str(&mut s, "=")?;
        let left = get(&mut s)?;
        let kind = get(&mut s)?;
        let right = get(&mut s)?;

        Ok(Operation { left, kind, right })
    }
}

#[derive(Debug)]
pub struct Test {
    divisibility: Item,
    if_true: MonkeyId,
    if_false: MonkeyId,
}

impl Test {
    pub fn perform_on(&self, item: Item) -> MonkeyId {
        if (item % self.divisibility) == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl<S> FromIterator<S> for Test
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut iter = iter.into_iter().map(|s| {
            s.as_ref()
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
        });

        let divisibility = iter.next().unwrap().parse().unwrap();
        let if_true = iter.next().unwrap().parse().unwrap();
        let if_false = iter.next().unwrap().parse().unwrap();

        Test {
            divisibility,
            if_true,
            if_false,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    inspect_count: Cell<Level>,
}

impl Monkey {
    fn become_bored(&self, item: Item) -> Item {
        item / 3
    }

    fn inspect(&self, item: Item, product: Option<Item>) -> (MonkeyId, Item) {
        self.inspect_count.set(self.inspect_count.get() + 1);
        let item = self.operation.apply_on(item);

        let item = if let Some(product) = product {
            item % product
        } else {
            self.become_bored(item)
        };

        (self.test.perform_on(item), item)
    }

    pub fn inspect_everything(&mut self, product: Option<Item>) -> Vec<(MonkeyId, Item)> {
        let result = self
            .items
            .iter()
            .copied()
            .map(|i| self.inspect(i, product))
            .collect::<Vec<_>>();
        self.items.clear();
        result
    }

    pub fn catch(&mut self, item: Item) {
        self.items.push(item);
    }
}

impl<S> FromIterator<S> for Monkey
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut iter = iter.into_iter().skip(1);

        let items = {
            let s = iter.next().unwrap();
            let s = s.as_ref();
            let (_, right) = s.split_once(": ").unwrap();
            right
                .split(", ")
                .map(|s| s.parse::<Item>().unwrap())
                .collect::<Vec<_>>()
        };

        let operation = {
            let s = iter.next().unwrap();
            let s = s.as_ref();
            let (_, right) = s.split_once(": ").unwrap();
            right.parse::<Operation>().unwrap()
        };

        let test = iter.collect::<Test>();

        Monkey {
            items,
            operation,
            test,
            inspect_count: Cell::new(0),
        }
    }
}

#[derive(Debug)]
pub struct Business {
    monkeys: Vec<Monkey>,
    divisors_product: Item,
}

impl Business {
    pub fn round(&mut self, use_product: bool) {
        for id in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[id];
            for (id, item) in
                monkey.inspect_everything(use_product.then_some(self.divisors_product))
            {
                self.monkeys[id].catch(item);
            }
        }
    }

    pub fn level(&self) -> Level {
        self.monkeys
            .iter()
            .map(|m| m.inspect_count.get())
            .sorted()
            .rev()
            .take(2)
            .product::<Level>()
    }

    pub fn level_after(&mut self, rounds: usize, use_product: bool) -> Level {
        for _ in 0..rounds {
            self.round(use_product);
        }
        self.level()
    }
}

impl<S> FromIterator<S> for Business
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let monkeys = iter
            .into_iter()
            .group_by(|s| s.as_ref().is_empty())
            .into_iter()
            .filter_map(|(t, m)| if t { None } else { Some(m.collect::<Monkey>()) })
            .collect::<Vec<_>>();

        let divisors_product = monkeys
            .iter()
            .map(|monkey| monkey.test.divisibility)
            .product::<Item>();

        Business {
            monkeys,
            divisors_product,
        }
    }
}

pub struct Day11 {
    dataset: Dataset,
}

impl Day11 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day11 {
    fn name(&self) -> &'static str {
        "Monkey in the Middle"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let mut business = data.iter().collect::<Business>();

        Ok(business.level_after(20, false).to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let mut business = data.iter().collect::<Business>();

        Ok(business.level_after(10000, true).to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
