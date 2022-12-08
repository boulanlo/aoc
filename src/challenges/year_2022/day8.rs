use std::ops::Index;

use color_eyre::Result;
use itertools::{FoldWhile, Itertools};

use crate::{runner::Messenger, Challenge, Dataset};

struct Canopy {
    trees: Vec<Vec<u8>>,
}

impl Canopy {
    pub fn visible_trees(&self) -> usize {
        let visible_from_border = (self.trees[0].len() * 2) + ((self.trees.len() - 2) * 2);

        let visible_inner = (1..self.trees[0].len() - 1)
            .flat_map(|x| (1..self.trees.len() - 1).map(move |y| (x, y)))
            .map(|(x, y)| self.is_visible(x, y) as usize)
            .sum::<usize>();

        visible_from_border + visible_inner
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree = self[(x, y)];
        self.line(y).take(x).all(|t| t < tree)
            || self.line(y).skip(x + 1).all(|t| t < tree)
            || self.column(x).take(y).all(|t| t < tree)
            || self.column(x).skip(y + 1).all(|t| t < tree)
    }

    fn max_scenic_score(&self) -> usize {
        (1..self.trees[0].len() - 1)
            .flat_map(|x| (1..self.trees.len() - 1).map(move |y| (x, y)))
            .map(|(x, y)| self.scenic_score(x, y))
            .max()
            .unwrap_or_default()
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree = self[(x, y)];
        let calculation = move |acc: usize, t| {
            if t < tree {
                FoldWhile::Continue(acc + 1)
            } else {
                FoldWhile::Done(acc + 1)
            }
        };

        let left = self
            .line(y)
            .take(x)
            .rev()
            .fold_while(0, calculation)
            .into_inner();

        let right = self
            .line(y)
            .skip(x + 1)
            .fold_while(0, calculation)
            .into_inner();

        let top = self
            .column(x)
            .take(y)
            .rev()
            .fold_while(0, calculation)
            .into_inner();

        let bottom = self
            .column(x)
            .skip(y + 1)
            .fold_while(0, calculation)
            .into_inner();

        top * bottom * left * right
    }

    fn line(&self, y: usize) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + '_ {
        self.trees[y].iter().copied()
    }

    fn column(&self, x: usize) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + '_ {
        (0..self.trees.len()).map(move |y| self[(x, y)])
    }
}

impl<S> FromIterator<S> for Canopy
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let trees = iter
            .into_iter()
            .map(|s| {
                s.as_ref()
                    .chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { trees }
    }
}

impl Index<(usize, usize)> for Canopy {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.trees.index(y).index(x)
    }
}

pub struct Day8 {
    dataset: Dataset,
}

impl Day8 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day8 {
    fn name(&self) -> &'static str {
        "Treetop Tree House"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let canopy = data.iter().collect::<Canopy>();

        Ok(canopy.visible_trees().to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let canopy = data.iter().collect::<Canopy>();

        Ok(canopy.max_scenic_score().to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
