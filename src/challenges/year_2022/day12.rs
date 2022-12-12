use std::ops::Index;

use color_eyre::Result;
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

type Position = (i64, i64);

fn manhattan_distance((x1, y1): Position, (x2, y2): Position) -> u64 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn is_negative((x, y): Position) -> bool {
    x < 0 || y < 0
}

struct Map {
    squares: Vec<Vec<u8>>,
    start: Position,
    end: Position,
}

impl Map {
    pub fn find_path(&self) -> Vec<Position> {
        let mut distances = vec![u64::MAX; self.width() * self.height()];
        let mut previous = vec![None; self.width() * self.height()];
        let mut q = (0..self.width() * self.height()).collect_vec();
        distances[self.idx(self.start)] = 0;

        while !q.is_empty() {
            let (u_idx, (u, _)) = q
                .iter()
                .map(|x| (*x, distances[*x]))
                .enumerate()
                .min_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
                .unwrap();

            // eprintln!("Considering {:?}", self.pos(u));
            // eprintln!("Prev = {previous:?}");

            if self.pos(u) == self.end {
                return std::iter::successors(Some(self.pos(u)), |p| {
                    previous[self.idx(*p)].map(|p| self.pos(p))
                })
                .collect_vec();
            }

            q.swap_remove(u_idx);

            for v in self.available_squares(self.pos(u)) {
                // eprintln!("  Now looking at {v:?}...");
                let v = self.idx(v);
                let alt = distances[u] + manhattan_distance(self.pos(v), self.end);

                if alt < distances[v] {
                    // eprintln!("  Found better distance!");
                    distances[v] = alt;
                    previous[v] = Some(u);
                }
            }

            // std::thread::sleep(Duration::from_millis(500))
        }

        todo!()
    }

    fn find_path_from_end(&self) -> Vec<Position> {
        let mut distances = vec![u64::MAX; self.width() * self.height()];
        let mut previous = vec![None; self.width() * self.height()];
        let mut q = (0..self.width() * self.height()).collect_vec();
        distances[self.idx(self.end)] = 0;

        while !q.is_empty() {
            let (u_idx, (u, _)) = q
                .iter()
                .map(|x| (*x, distances[*x]))
                .enumerate()
                .min_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
                .unwrap();

            q.swap_remove(u_idx);

            for v in self.available_squares_descent(self.pos(u)) {
                let v = self.idx(v);
                let alt = distances[u].saturating_add(manhattan_distance(self.end, self.pos(u)));

                if alt < distances[v] {
                    distances[v] = alt;
                    previous[v] = Some(u);
                }
            }
        }

        self.all_zeros()
            .map(|a| {
                std::iter::successors(Some(self.pos(a)), |p| {
                    previous[self.idx(*p)].map(|p| self.pos(p))
                })
                .collect_vec()
            })
            .filter(|v| v.contains(&self.end))
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
    }

    fn all_zeros(&self) -> impl Iterator<Item = usize> + '_ {
        self.squares.iter().enumerate().flat_map(move |(y, l)| {
            l.iter().enumerate().filter_map(move |(x, v)| {
                if *v == 0 {
                    Some(self.idx((x as i64, y as i64)))
                } else {
                    None
                }
            })
        })
    }

    fn idx(&self, (x, y): Position) -> usize {
        (y as usize) * self.width() + x as usize
    }

    fn pos(&self, idx: usize) -> Position {
        let x = idx % self.width();
        let y = idx / self.width();
        (x as i64, y as i64)
    }

    fn width(&self) -> usize {
        self.squares[0].len()
    }

    fn height(&self) -> usize {
        self.squares.len()
    }

    fn available_squares_descent(&self, (x, y): Position) -> impl Iterator<Item = Position> + '_ {
        let current = self[(x, y)];
        [(1, 0), (-1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(move |(x1, y1)| {
                let p = (x + x1, y + y1);
                if is_negative(p)
                    || p.0 >= self.width() as i64
                    || p.1 >= self.height() as i64
                    || self[p] < current.saturating_sub(1)
                {
                    None
                } else {
                    Some(p)
                }
            })
    }

    fn available_squares(&self, (x, y): Position) -> impl Iterator<Item = Position> + '_ {
        let current = self[(x, y)];
        [(1, 0), (-1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(move |(x1, y1)| {
                let p = (x + x1, y + y1);
                if is_negative(p)
                    || p.0 >= self.width() as i64
                    || p.1 >= self.height() as i64
                    || self[p] > current + 1
                {
                    None
                } else {
                    Some(p)
                }
            })
    }
}

impl Index<Position> for Map {
    type Output = u8;

    fn index(&self, (x, y): Position) -> &Self::Output {
        self.squares.index(y as usize).index(x as usize)
    }
}

impl<S> FromIterator<S> for Map
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut start = None;
        let mut end = None;

        let squares = iter
            .into_iter()
            .enumerate()
            .map(|(y, s)| {
                s.as_ref()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some((x as _, y as _));
                            0
                        }
                        'E' => {
                            end = Some((x as _, y as _));
                            b'z' - b'a'
                        }
                        'a'..='z' => c as u8 - b'a',
                        _ => panic!("invalid char {c}"),
                    })
                    .collect()
            })
            .collect();

        Map {
            squares,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}

pub struct Day12 {
    dataset: Dataset,
}

impl Day12 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day12 {
    fn name(&self) -> &'static str {
        "Hill Climbing Algorithm"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let map = data.iter().collect::<Map>();

        let path = map.find_path();
        Ok((path.len() - 1).to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let map = data.iter().collect::<Map>();

        let path = map.find_path_from_end();

        Ok((path.len() - 1).to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
