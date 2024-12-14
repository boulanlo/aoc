use std::{cmp::Ordering, convert::Infallible, str::FromStr};

use aoc_utils::*;
use image::ImageBuffer;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

impl FromStr for Robot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').unwrap();
        let (_, p) = p.split_once('=').unwrap();
        let (x, y) = p.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

        let (_, v) = v.split_once('=').unwrap();
        let (dx, dy) = v.split_once(',').unwrap();
        let (dx, dy) = (dx.parse().unwrap(), dy.parse().unwrap());

        Ok(Self {
            position: (x, y),
            velocity: (dx, dy),
        })
    }
}

#[derive(Debug, Clone)]
struct Room {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl Room {
    pub fn new(s: &str, width: usize, height: usize) -> Self {
        Self {
            robots: s.lines().map(|s| s.parse().unwrap()).collect(),
            width,
            height,
        }
    }

    pub fn after_time(&mut self, seconds: usize) {
        for r in self.robots.iter_mut() {
            let (nx, ny) = (
                (r.position.0 as isize + (r.velocity.0 * seconds as isize))
                    .rem_euclid(self.width as isize),
                (r.position.1 as isize + (r.velocity.1 * seconds as isize))
                    .rem_euclid(self.height as isize),
            );

            r.position.0 = nx as usize;
            r.position.1 = ny as usize;
        }
    }

    pub fn safety_factor(&self) -> usize {
        self.robots
            .iter()
            .fold([0, 0, 0, 0], |mut a, r| {
                match (
                    r.position.0.cmp(&(self.width / 2)),
                    r.position.1.cmp(&(self.height / 2)),
                ) {
                    (Ordering::Less, Ordering::Less) => a[0] += 1,
                    (Ordering::Less, Ordering::Greater) => a[1] += 1,
                    (Ordering::Greater, Ordering::Less) => a[2] += 1,
                    (Ordering::Greater, Ordering::Greater) => a[3] += 1,
                    _ => {}
                }

                a
            })
            .into_iter()
            .product()
    }

    pub fn images(&mut self, i: usize) {
        let mut imgbuf;

        std::fs::create_dir("./day_2024_14_images").unwrap();

        for i in 0..i {
            imgbuf = image::ImageBuffer::new((self.width + 1) as u32, (self.height + 1) as u32);
            for r in &self.robots {
                imgbuf[(r.position.0 as u32, r.position.1 as u32)] =
                    image::Rgb([0xFFu8, 0xFF, 0xFF]);
            }

            imgbuf
                .save(format!("./day_2024_14_images/{i:05}.png"))
                .unwrap();

            self.after_time(1);
        }
    }
}

fn part_1(input: &str) {
    let mut room = Room::new(input, 101, 103);

    room.after_time(100);

    let result = room.safety_factor();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let mut room = Room::new(input, 101, 103);

    room.images(10000);

    println!("Part 2: Check the day_2024_14_images folder.");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
