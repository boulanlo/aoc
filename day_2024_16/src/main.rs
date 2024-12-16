use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    convert::Infallible,
    str::FromStr,
};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(usize)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn vector(&self) -> (isize, isize) {
        [(1, 0), (0, 1), (-1, 0), (0, -1)][*self as usize]
    }

    pub fn distance(&self, other: Self) -> usize {
        if *self == other {
            0
        } else if (*self as usize).abs_diff(other as usize) % 2 == 1 {
            1
        } else {
            2
        }
    }
}

struct Maze {
    tiles: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Maze {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;

        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.char_indices()
                    .map(|(x, c)| match c {
                        '.' => false,
                        '#' => true,
                        'S' => {
                            start = Some((x, y));
                            false
                        }
                        'E' => {
                            end = Some((x, y));
                            false
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            tiles,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

impl Maze {
    pub fn surrounding_tiles(
        &self,
        x: usize,
        y: usize,
        d: Direction,
    ) -> impl Iterator<Item = ((usize, usize), usize, Direction)> + '_ {
        [
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ]
        .into_iter()
        .filter_map(move |od| {
            let (dx, dy) = od.vector();

            x.checked_add_signed(dx).and_then(|x| {
                y.checked_add_signed(dy).and_then(|y| {
                    if self.tiles[y][x] {
                        None
                    } else {
                        Some(((x, y), d.distance(od) * 1000, od))
                    }
                })
            })
        })
    }

    pub fn score(&self) -> (usize, usize) {
        let mut visited = HashMap::<
            ((usize, usize), Direction),
            (usize, HashSet<((usize, usize), Direction)>),
        >::new();
        visited.insert((self.start, Direction::East), (0, HashSet::new()));

        let mut queue = VecDeque::new();
        queue.push_back((self.start, Direction::East));

        while let Some(((x, y), d)) = queue.pop_front() {
            let (current_cost, _) = visited.get(&((x, y), d)).unwrap();
            let current_cost = *current_cost;
            for ((nx, ny), turning_cost, dir) in self.surrounding_tiles(x, y, d) {
                if let Some((cost, _)) = visited.get(&((nx, ny), dir)) {
                    match cost.cmp(&(current_cost + turning_cost + 1)) {
                        Ordering::Greater => {
                            visited.insert(
                                ((nx, ny), dir),
                                (
                                    current_cost + turning_cost + 1,
                                    [((x, y), d)].into_iter().collect(),
                                ),
                            );
                            queue.push_back(((nx, ny), dir));
                        }
                        Ordering::Equal => {
                            visited
                                .get_mut(&((nx, ny), dir))
                                .unwrap()
                                .1
                                .insert(((x, y), d));
                        }
                        Ordering::Less => {}
                    }
                } else {
                    visited.insert(
                        ((nx, ny), dir),
                        (
                            current_cost + turning_cost + 1,
                            [((x, y), d)].into_iter().collect(),
                        ),
                    );
                    queue.push_back(((nx, ny), dir));
                }
            }
        }

        let (res, prevs) = visited
            .iter()
            .filter_map(|((xy, _), (v, p))| {
                if *xy == self.end {
                    Some((*v, p.clone()))
                } else {
                    None
                }
            })
            .min_by_key(|(v, _)| *v)
            .unwrap();

        let path = std::iter::successors(Some(prevs), |p| {
            if p.is_empty() {
                None
            } else {
                Some(
                    p.iter()
                        .flat_map(|k| visited.get(k).unwrap().1.iter().copied())
                        .collect(),
                )
            }
        })
        .fold(HashSet::new(), |mut h, x| {
            h.extend(x.into_iter().map(|(xy, _)| xy));
            h
        });

        (res, path.len() + 1)
    }
}

fn part_1(input: &str) {
    let maze: Maze = input.parse().unwrap();

    let result = maze.score().0;

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let maze: Maze = input.parse().unwrap();

    let result = maze.score().1;

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
