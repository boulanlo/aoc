use std::{
    collections::{HashSet, VecDeque},
    convert::Infallible,
    str::FromStr,
};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

struct Map {
    heights: Vec<Vec<u8>>,
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            heights: s
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| if c == '.' { 10 } else { c as u8 - b'0' })
                        .collect()
                })
                .collect(),
        })
    }
}

impl Map {
    fn width(&self) -> usize {
        self.heights[0].len()
    }

    fn height(&self) -> usize {
        self.heights.len()
    }

    fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let val = self.heights[y][x];
        [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                x.checked_add_signed(dx).and_then(|x| {
                    y.checked_add_signed(dy).and_then(|y| {
                        (x < self.width()
                            && y < self.height()
                            && self.heights[y][x] == val + 1
                            && self.heights[y][x] < 10)
                            .then_some((x, y))
                    })
                })
            })
    }

    fn starting_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.heights.iter().enumerate().flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(x, h)| if *h == 0 { Some((x, y)) } else { None })
        })
    }

    pub fn trailhead_score(&self) -> usize {
        self.starting_points()
            .map(|(x, y)| {
                let mut visited = HashSet::new();
                let mut queue = self.neighbours(x, y).collect::<VecDeque<_>>();
                let mut found = 0usize;

                while !queue.is_empty() {
                    let (x, y) = queue.pop_front().unwrap();
                    if visited.insert((x, y)) {
                        if self.heights[y][x] == 9 {
                            found += 1;
                        }
                        queue.extend(self.neighbours(x, y));
                    }
                }

                found
            })
            .sum::<usize>()
    }

    pub fn trailhead_rating(&self) -> usize {
        self.starting_points()
            .map(|(x, y)| {
                let mut queue = self.neighbours(x, y).collect::<VecDeque<_>>();
                let mut rating = 0;

                while !queue.is_empty() {
                    let (x, y) = queue.pop_back().unwrap();
                    if self.heights[y][x] == 9 {
                        rating += 1;
                    }
                    queue.extend(self.neighbours(x, y));
                }

                rating
            })
            .sum::<usize>()
    }
}

fn part_1(input: &str) {
    let map: Map = input.parse().unwrap();

    let result = map.trailhead_score();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let map: Map = input.parse().unwrap();

    let result = map.trailhead_rating();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
