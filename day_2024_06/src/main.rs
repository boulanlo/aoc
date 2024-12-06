use std::{collections::HashSet, convert::Infallible, ops::ControlFlow, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

enum Tile {
    Empty,
    Obstacle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn next(&self, x: usize, y: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => y.checked_sub(1).map(|y| (x, y)),
            Direction::Right => (x != w - 1).then_some((x + 1, y)),
            Direction::Down => (y != h - 1).then_some((x, y + 1)),
            Direction::Left => x.checked_sub(1).map(|x| (x, y)),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

type Position = ((usize, usize), Direction);

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Position,
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;

        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.char_indices()
                    .map(|(x, c)| match c {
                        '.' => Tile::Empty,
                        c @ ('^' | '>' | 'v' | '<') => {
                            start = Some(((x, y), c.into()));
                            Tile::Empty
                        }
                        '#' => Tile::Obstacle,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            tiles,
            start: start.unwrap(),
        })
    }
}

fn is_looping<I>(i: I) -> bool
where
    I: IntoIterator<Item = Position>,
{
    i.into_iter()
        .try_fold(HashSet::with_capacity(1024), |mut h, p| {
            if !h.insert(p) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(h)
            }
        })
        .is_break()
}

impl Map {
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn guard_path(
        &self,
        start: Position,
        patch: Option<(usize, usize)>,
    ) -> impl Iterator<Item = Position> + '_ {
        std::iter::successors(Some(start), move |((x, y), dir)| {
            dir.next(*x, *y, self.width(), self.height())
                .map(|(next_x, next_y)| {
                    if matches!(self.tiles[next_y][next_x], Tile::Obstacle)
                        || patch
                            .map(|(px, py)| px == next_x && py == next_y)
                            .unwrap_or(false)
                    {
                        ((*x, *y), dir.rotate_right())
                    } else {
                        ((next_x, next_y), *dir)
                    }
                })
        })
    }

    fn possible_obstacles(&self, start: Position) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.guard_path(start, None)
            .tuple_windows()
            .filter(|&(_, ((x, y), _))| is_looping(self.guard_path(self.start, Some((x, y)))))
            .map(|(_, ((x, y), _))| (x, y))
    }
}

fn part_1(input: &str) {
    let map: Map = input.parse().unwrap();

    let result = map
        .guard_path(map.start, None)
        .map(|((x, y), _)| (x, y))
        .collect::<HashSet<_>>()
        .len();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let map: Map = input.parse().unwrap();

    let result = map
        .possible_obstacles(map.start)
        .collect::<HashSet<_>>()
        .len();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
