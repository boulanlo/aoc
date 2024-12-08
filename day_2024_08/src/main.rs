use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

struct Map {
    antennas: HashMap<char, Vec<(isize, isize)>>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();

        let antennas = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.char_indices()
                    .filter_map(move |(x, c)| (c != '.').then_some((c, (x as isize, y as isize))))
            })
            .fold(HashMap::new(), |mut h, (c, p)| {
                h.entry(c)
                    .and_modify(|v: &mut Vec<(isize, isize)>| v.push(p))
                    .or_insert_with(|| vec![p]);
                h
            });

        Ok(Map {
            antennas,
            width,
            height,
        })
    }
}

impl Map {
    fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    fn antinodes(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.antennas
            .values()
            .flat_map(|v| {
                v.iter().copied().combinations(2).flat_map(|v| {
                    let (xa, ya) = v[0];
                    let (xb, yb) = v[1];

                    let dx = xa - xb;
                    let dy = ya - yb;

                    [(xa + dx, ya + dy), (xb - dx, yb - dy)]
                })
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .filter_map(|(x, y)| self.is_in_bounds(x, y).then_some((x as usize, y as usize)))
    }

    fn antinodes_resonance(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.antennas
            .values()
            .flat_map(|v| {
                v.iter().copied().combinations(2).flat_map(|v| {
                    let (xa, ya) = v[0];
                    let (xb, yb) = v[1];

                    let dx = xa - xb;
                    let dy = ya - yb;

                    std::iter::successors(Some((xa, ya)), move |(xa, ya)| {
                        let (xa, ya) = (*xa + dx, *ya + dy);
                        self.is_in_bounds(xa, ya).then_some((xa, ya))
                    })
                    .chain(std::iter::successors(
                        Some((xb, yb)),
                        move |(xb, yb)| {
                            let (xb, yb) = (*xb - dx, *yb - dy);
                            self.is_in_bounds(xb, yb).then_some((xb, yb))
                        },
                    ))
                })
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|(x, y)| (x as usize, y as usize))
    }
}

fn part_1(input: &str) {
    let map: Map = input.parse().unwrap();

    let result = map.antinodes().count();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let map: Map = input.parse().unwrap();

    let result = map.antinodes_resonance().count();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
