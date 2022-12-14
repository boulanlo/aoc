use std::{fmt, str::FromStr};

use color_eyre::{Report, Result};
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn to_idx(&self, top_left: &Self, bottom_right: &Self) -> usize {
        let x = (self.x - top_left.x) as usize;
        let y = (self.y - top_left.y) as usize;

        let width = ((bottom_right.x - top_left.x) as usize) + 1;

        let idx = (y * width) + x;

        idx
    }

    pub fn down(&self) -> Self {
        (self.x, self.y + 1).into()
    }

    pub fn down_left(&self) -> Self {
        (self.x - 1, self.y + 1).into()
    }

    pub fn down_right(&self) -> Self {
        (self.x + 1, self.y + 1).into()
    }

    pub fn is_in_bounds(&self, top_left: &Self, bottom_right: &Self) -> bool {
        self.x >= top_left.x
            && self.x <= bottom_right.x
            && self.y >= top_left.y
            && self.y <= bottom_right.y
    }

    pub fn points_between(&self, other: &Self) -> Vec<Self> {
        if self.x == other.x {
            (self.y.min(other.y)..=self.y.max(other.y))
                .map(|y| (self.x, y).into())
                .collect()
        } else if self.y == other.y {
            (self.x.min(other.x)..=self.x.max(other.x))
                .map(|x| (x, self.y).into())
                .collect()
        } else {
            unreachable!()
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| color_eyre::eyre::eyre!("invalid point format"))?;

        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

struct Path {
    points: Vec<Point>,
}

impl FromStr for Path {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|s| s.parse::<Point>())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { points })
    }
}

struct Paths {
    paths: Vec<Path>,
}

impl Paths {
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.paths.iter().flat_map(|p| p.points.iter().copied())
    }

    pub fn top_left(&self) -> Point {
        let (x, y): (Vec<_>, Vec<_>) = self.points().map(|p| (p.x, p.y)).unzip();

        (x.into_iter().min().unwrap(), y.into_iter().min().unwrap()).into()
    }

    pub fn bottom_right(&self) -> Point {
        let (x, y): (Vec<_>, Vec<_>) = self.points().map(|p| (p.x, p.y)).unzip();

        (x.into_iter().max().unwrap(), y.into_iter().max().unwrap()).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Empty,
    Rock,
    Sand,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Empty => '.',
                Element::Rock => '#',
                Element::Sand => 'o',
            }
        )
    }
}

struct Cave {
    elements: Vec<Element>,
    top_left: Point,
    bottom_right: Point,
}

impl Cave {
    pub fn get(&self, p: Point) -> Element {
        if p.is_in_bounds(&self.top_left, &self.bottom_right) {
            self.elements[p.to_idx(&self.top_left, &self.bottom_right)]
        } else {
            Element::Empty
        }
    }

    pub fn fallen_out(&self, p: Point) -> bool {
        p.x < self.top_left.x || p.x > self.bottom_right.x || p.y > self.bottom_right.y
    }

    pub fn is_at_bottom(&self, p: Point) -> bool {
        p.y == self.bottom_right.y - 1
    }

    pub fn extend(&mut self, p: Point) {
        let (start, dir) = if p.x < self.top_left.x {
            (0, -1)
        } else {
            ((self.bottom_right.x - self.top_left.x + 1) as usize, 1)
        };

        while !p.is_in_bounds(&self.top_left, &self.bottom_right) {
            for idx in (start..self.elements.len() + 1)
                .step_by((self.bottom_right.x - self.top_left.x + 1) as usize)
                .rev()
            {
                self.elements.insert(idx, Element::Empty);
            }

            if dir < 0 {
                self.top_left.x -= 1;
            } else {
                self.bottom_right.x += 1;
            }
        }
    }

    pub fn fill_with_sand(&mut self, start: Point) -> usize {
        enum State {
            Falling,
            Settled(Point),
            OutOfBounds,
        }

        std::iter::repeat(start)
            .enumerate()
            .find_map(|(i, start)| {
                match std::iter::successors(Some((start, State::Falling)), |(p, _)| {
                    if self.fallen_out(*p) {
                        Some((*p, State::OutOfBounds))
                    } else if let Element::Empty = self.get(p.down()) {
                        Some((p.down(), State::Falling))
                    } else if let Element::Empty = self.get(p.down_left()) {
                        Some((p.down_left(), State::Falling))
                    } else if let Element::Empty = self.get(p.down_right()) {
                        Some((p.down_right(), State::Falling))
                    } else {
                        Some((*p, State::Settled(*p)))
                    }
                })
                .find(|(_, s)| !matches!(s, State::Falling))
                .unwrap()
                .1
                {
                    State::Falling => unreachable!(),
                    State::Settled(p) => {
                        debug_assert!(p.is_in_bounds(&self.top_left, &self.bottom_right));
                        self.elements[p.to_idx(&self.top_left, &self.bottom_right)] = Element::Sand;
                        None
                    }
                    State::OutOfBounds => Some(i),
                }
            })
            .unwrap()
    }

    pub fn fill_with_sand_on_floor(&mut self, start: Point) -> usize {
        enum State {
            Falling,
            Settled(Point),
        }

        self.elements.extend(
            std::iter::repeat(Element::Empty)
                .take((self.bottom_right.x - self.top_left.x + 1) as usize),
        );
        self.bottom_right.y += 2;

        std::iter::repeat(start)
            .enumerate()
            .find_map(|(i, start)| {
                match std::iter::successors(Some((start, State::Falling)), |(p, _)| {
                    if self.is_at_bottom(*p) {
                        Some((*p, State::Settled(*p)))
                    } else if let Element::Empty = self.get(p.down()) {
                        Some((p.down(), State::Falling))
                    } else if let Element::Empty = self.get(p.down_left()) {
                        Some((p.down_left(), State::Falling))
                    } else if let Element::Empty = self.get(p.down_right()) {
                        Some((p.down_right(), State::Falling))
                    } else {
                        Some((*p, State::Settled(*p)))
                    }
                })
                .find(|(_, s)| !matches!(s, State::Falling))
                .unwrap()
                .1
                {
                    State::Falling => unreachable!(),
                    State::Settled(p) => {
                        if p == start {
                            Some(i + 1)
                        } else {
                            if !p.is_in_bounds(&self.top_left, &self.bottom_right) {
                                self.extend(p);
                            }
                            self.elements[p.to_idx(&self.top_left, &self.bottom_right)] =
                                Element::Sand;
                            None
                        }
                    }
                }
            })
            .unwrap()
    }
}

impl<S> FromIterator<S> for Cave
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let paths = Paths {
            paths: iter
                .into_iter()
                .map(|s| s.as_ref().parse::<Path>().unwrap())
                .collect(),
        };

        let mut top_left = paths.top_left();
        top_left.y = 0;
        let bottom_right = paths.bottom_right();

        let size =
            (((bottom_right.x - top_left.x) + 1) * ((bottom_right.y - top_left.y) + 1)) as usize;

        let mut elements = vec![Element::Empty; size];

        paths.paths.iter().for_each(|p| {
            p.points.iter().tuple_windows().for_each(|(a, b)| {
                for p in a.points_between(b) {
                    elements[p.to_idx(&top_left, &bottom_right)] = Element::Rock;
                }
            });
        });

        Self {
            elements,
            top_left,
            bottom_right,
        }
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.elements
            .chunks((self.bottom_right.x - self.top_left.x + 1) as usize)
            .try_for_each(|l| {
                writeln!(f, "{}", l.iter().map(|e| e.to_string()).collect::<String>())
            })
    }
}

pub struct Day14 {
    dataset: Dataset,
}

impl Day14 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day14 {
    fn name(&self) -> &'static str {
        "Regolith Reservoir"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let mut cave = data.iter().collect::<Cave>();

        let i = cave.fill_with_sand((500, 0).into());

        Ok(i.to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let mut cave = data.iter().collect::<Cave>();

        let i = cave.fill_with_sand_on_floor((500, 0).into());

        Ok(i.to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
