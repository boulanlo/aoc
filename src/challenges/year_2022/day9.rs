use std::{collections::HashSet, str::FromStr};

use color_eyre::{Report, Result};
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

pub struct Day9 {
    dataset: Dataset,
}

impl Day9 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn move_towards(&mut self, other: Self) {
        let (dist_x, dist_y) = (self.x - other.x, self.y - other.y);

        match (dist_x, dist_y) {
            (2 | -2, 0) => self.x -= dist_x.signum(),
            (0, 2 | -2) => self.y -= dist_y.signum(),
            // If you're trying to optimise this later, sorry.
            // It's 07:23, I just solved this, and I'm not even sure how this works right now.
            (1 | -1 | -2 | 2, 2 | -2) | (2 | -2, 1 | -1) => {
                self.x -= dist_x.signum();
                self.y -= dist_y.signum()
            }
            _ => {}
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

struct Rope {
    knots: Vec<Position>,
    tail_positions: HashSet<Position>,
}

impl Rope {
    pub fn new(n: usize) -> Self {
        Self {
            knots: vec![(0, 0).into(); n],
            tail_positions: HashSet::default(),
        }
    }

    pub fn apply_move(&mut self, m: Move) {
        let (dir_vector, amount) = match m {
            Move::Left(a) => ((-1, 0), a),
            Move::Up(a) => ((0, -1), a),
            Move::Down(a) => ((0, 1), a),
            Move::Right(a) => ((1, 0), a),
        };

        for _ in 0..amount {
            self.knots[0].x += dir_vector.0;
            self.knots[0].y += dir_vector.1;

            for (head_idx, tail_idx) in (0..self.knots.len()).tuple_windows() {
                let tail = self.knots[head_idx];
                self.knots[tail_idx].move_towards(tail);
            }

            self.tail_positions.insert(*self.knots.last().unwrap());
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left(i32),
    Up(i32),
    Down(i32),
    Right(i32),
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s
            .split_once(' ')
            .ok_or_else(|| color_eyre::eyre::eyre!("Invalid input format"))?;

        let amount = amount.parse()?;

        Ok(match dir {
            "U" => Move::Up(amount),
            "D" => Move::Down(amount),
            "L" => Move::Left(amount),
            "R" => Move::Right(amount),
            _ => return Err(color_eyre::eyre::eyre!("Invalid move direction: {dir}")),
        })
    }
}

impl Challenge for Day9 {
    fn name(&self) -> &'static str {
        "Rope Bridge"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let moves = data
            .iter()
            .map(|s| s.parse::<Move>())
            .try_fold(Rope::new(2), |mut r, m| {
                m.map(|m| {
                    r.apply_move(m);
                    r
                })
            })?
            .tail_positions;

        Ok(moves.len().to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let moves = data
            .iter()
            .map(|s| s.parse::<Move>())
            .try_fold(Rope::new(10), |mut r, m| {
                m.map(|m| {
                    r.apply_move(m);
                    r
                })
            })?
            .tail_positions;

        Ok(moves.len().to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
