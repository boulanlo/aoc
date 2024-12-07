use std::{collections::HashSet, str::FromStr};

use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: isize,
    color: Color,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let d = tokens.next().unwrap();
        let n = tokens.next().unwrap();
        let c = tokens.next().unwrap();
        assert_eq!(tokens.next(), None);

        let c = u32::from_str_radix(c.split_once('#').unwrap().1.split_once(')').unwrap().0, 16)
            .unwrap();

        Ok(Instruction {
            direction: match d {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => unreachable!(),
            },
            amount: n.parse().unwrap(),
            color: Color {
                r: ((c >> 16) & 0xFF) as u8,
                g: ((c >> 8) & 0xFF) as u8,
                b: (c & 0xFF) as u8,
            },
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
enum Alignment {
    Horizontal,
    Vertical,
}

impl From<Direction> for Alignment {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up | Direction::Down => Alignment::Vertical,
            Direction::Right | Direction::Left => Alignment::Horizontal,
        }
    }
}

#[derive(Debug)]
struct Edge {
    from: Point,
    to: Point,
    color: Color,
    alignment: Alignment,
}

#[derive(Debug)]
enum Intersection {
    Horizontal(isize, isize),
    Vertical(isize),
}

#[derive(Debug)]
struct DigPlan {
    edges: Vec<Edge>,
}

impl DigPlan {
    pub fn new<I>(instructions: I) -> Self
    where
        I: IntoIterator<Item = Instruction>,
    {
        let mut last = Point { x: 0, y: 0 };
        let mut edges = Vec::new();

        for i in instructions.into_iter() {
            edges.push(Edge {
                from: last,
                to: match i.direction {
                    Direction::Up => (last.x, last.y - i.amount),
                    Direction::Right => (last.x + i.amount, last.y),
                    Direction::Down => (last.x, last.y + i.amount),
                    Direction::Left => (last.x - i.amount, last.y),
                }
                .into(),
                color: i.color,
                alignment: i.direction.into(),
            });
            last = edges.last().unwrap().to;
        }

        DigPlan { edges }
    }

    pub fn edges(&self) -> Vec<Vec<Intersection>> {
        let top = self.edges.iter().min_by_key(|e| e.from.y).unwrap().from.y;
        let bottom = self.edges.iter().max_by_key(|e| e.from.y).unwrap().from.y;

        println!("{top:?}, {bottom:?}");

        let mut intersections = Vec::new();

        for y in top..=bottom {
            println!("At Y = {y}:");

            let mut i = Vec::new();

            for e in self.edges.iter().filter(|e| match e.alignment {
                Alignment::Horizontal => e.from.y == y,
                Alignment::Vertical => (e.from.y.min(e.to.y)..=e.from.y.max(e.to.y)).contains(&y),
            }) {
                println!("  {e:?}");

                match e.alignment {
                    Alignment::Horizontal => {
                        let mut x1 = e.from.x.min(e.to.x);
                        let mut x2 = e.from.x.max(e.to.x);

                        if let Some((a, b)) = i.iter().find_map(|x| match x {
                            Intersection::Horizontal(a, b) => {
                                if *a == x2 || *b == x1 {
                                    Some((*a, *b))
                                } else {
                                    None
                                }
                            }
                            Intersection::Vertical(_) => None,
                        }) {
                            if a == x2 {
                                x1 = b;
                            }
                            if b == x1 {
                                x2 = a;
                            }
                        }

                        i.retain(|x| match x {
                            Intersection::Horizontal(_, _) => true,
                            Intersection::Vertical(x) => !(x1..=x2).contains(x),
                        });

                        let e = Intersection::Horizontal(x1, x2);
                        i.push(e);
                    }
                    Alignment::Vertical => {
                        let x1 = e.to.x;
                        if !i.iter().any(|x| match x {
                            Intersection::Horizontal(a, b) => (*a..=*b).contains(&x1),
                            Intersection::Vertical(x) => *x == x1,
                        }) {
                            i.push(Intersection::Vertical(x1));
                        }
                    }
                }
            }

            i.sort_unstable_by(|x, y| {
                let x = match x {
                    Intersection::Horizontal(_, x) => x,
                    Intersection::Vertical(x) => x,
                };
                let y = match y {
                    Intersection::Horizontal(_, y) => y,
                    Intersection::Vertical(y) => y,
                };
                x.cmp(y)
            });

            intersections.push(i);
        }

        intersections
    }
}

fn part_1(input: &str) {
    let instructions = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Instruction>>();

    let plan = DigPlan::new(instructions);
    println!("{plan:?}");

    let edges = plan.edges();
    println!("{edges:?}");

    let res = edges
        .into_iter()
        .map(|l| match l.as_slice() {
            [] => 0,
            [Intersection::Vertical(_)] => unreachable!(),
            [Intersection::Horizontal(a, b)] => a.abs_diff(*b) + 1,
            _ => {
                if l.len() % 2 == 1 {
                    unreachable!()
                }

                l.into_iter()
                    .tuples()
                    .map(|(a, b)| match (a, b) {
                        (Intersection::Horizontal(a1, a2), Intersection::Horizontal(b1, b2)) => {
                            a1.abs_diff(a2) + 1 + b1.abs_diff(b2) + 1
                        }
                        (Intersection::Horizontal(a1, _), Intersection::Vertical(b)) => {
                            a1.abs_diff(b) + 1
                        }
                        (Intersection::Vertical(a), Intersection::Horizontal(_, b2)) => {
                            a.abs_diff(b2) + 1
                        }
                        (Intersection::Vertical(a), Intersection::Vertical(b)) => a.abs_diff(b) + 1,
                    })
                    .sum::<usize>()
            }
        })
        .sum::<usize>();

    println!("{res}")
}

fn part_2(input: &str) {
    // part 2
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    //part_2(input);
}
