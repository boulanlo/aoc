use std::ops::Index;

use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"30373
25512
65332
33549
35390
"#;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    trees: Vec<u8>,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.chars().filter(|c| *c == '\n').count();

        Self {
            width,
            height,
            trees: input
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_digit() {
                        Some((c as u8) - b'0')
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    fn lookups(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
        [
            // North
            (0..y).rev().map(|y| (x, y)).collect::<Vec<_>>().into_iter(),
            // East
            ((x + 1).min(self.width)..self.width)
                .map(|x| (x, y))
                .collect::<Vec<_>>()
                .into_iter(),
            // South
            ((y + 1).min(self.height)..self.height)
                .map(|y| (x, y))
                .collect::<Vec<_>>()
                .into_iter(),
            // West
            (0..x).rev().map(|x| (x, y)).collect::<Vec<_>>().into_iter(),
        ]
        .into_iter()
    }

    fn visible(&self) -> usize {
        (0..self.trees.len())
            .filter_map(|i| {
                let (x, y) = (i % self.width, i / self.width);
                let tree = self[(x, y)];

                if self
                    .lookups((x, y))
                    .any(|mut i| i.all(|(x, y)| self[(x, y)] < tree))
                {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    }

    fn max_scenic_score(&self) -> usize {
        (0..self.trees.len())
            .map(|i| {
                let (x, y) = (i % self.width, i / self.width);
                let tree = self[(x, y)];

                self.lookups((x, y))
                    .map(|mut i| {
                        i.take_while_inclusive(|(x, y)| self[(*x, *y)] < tree)
                            .count()
                    })
                    .product()
            })
            .max()
            .unwrap()
    }
}

impl Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let idx = y * self.width + x;
        self.trees.index(idx)
    }
}

fn part_1(input: &str) {
    let map = Map::new(input);
    println!("{}", map.visible());
}

fn part_2(input: &str) {
    let map = Map::new(input);
    println!("{}", map.max_scenic_score());
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
