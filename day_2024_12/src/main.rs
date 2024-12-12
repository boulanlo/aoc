use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ops::Range,
    str::FromStr,
};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Horizontal(usize, Direction),
    Vertical(usize, Direction),
}

fn merge_range_array(v: &mut Vec<Range<usize>>) {
    let mut v2 = Vec::new();
    core::mem::swap(&mut v2, v);
    *v = v2.into_iter().fold(Vec::new(), |mut v, r| {
        if let Some(last) = v.last_mut() {
            if last.contains(&r.start) || last.end == r.start {
                last.end = r.end;
            } else {
                v.push(r);
            }
        } else {
            v.push(r);
        }

        v
    });
}

#[allow(clippy::single_range_in_vec_init)]
fn calculate_sides(h: &HashMap<(usize, usize), HashSet<Direction>>) -> usize {
    let h = h.iter().fold(
        HashMap::<Side, Vec<Range<usize>>>::new(),
        |mut h, (&(x, y), d)| {
            for d in d {
                let side = match d {
                    Direction::Up => Side::Horizontal(y, *d),
                    Direction::Right => Side::Vertical(x + 1, *d),
                    Direction::Down => Side::Horizontal(y + 1, *d),
                    Direction::Left => Side::Vertical(x, *d),
                };

                let r = match side {
                    Side::Horizontal(_, _) => x..x + 1,
                    Side::Vertical(_, _) => y..y + 1,
                };

                h.entry(side)
                    .and_modify(|v| {
                        let r = r.clone();
                        v.push(r);
                        v.sort_by_cached_key(|r| r.start);
                        merge_range_array(v);
                    })
                    .or_insert_with(|| vec![r]);
            }

            h
        },
    );

    h.into_values().map(|v| v.len()).sum()
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn lookups(&self) -> impl Iterator<Item = (isize, isize)> {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .cycle()
            .skip((*self as u8) as usize)
            .take(4)
    }

    fn from_vector(ax: usize, ay: usize, bx: usize, by: usize) -> Self {
        let dx = bx as isize - ax as isize;
        let dy = by as isize - ay as isize;

        match (dx, dy) {
            (-1, 0) => Direction::Left,
            (0, -1) => Direction::Up,
            (1, 0) => Direction::Right,
            (0, 1) => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

struct Garden {
    tiles: Vec<Vec<char>>,
}

impl FromStr for Garden {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Garden {
            tiles: s.lines().map(|l| l.chars().collect()).collect(),
        })
    }
}

impl Garden {
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn all_coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
    }

    pub fn adjacent(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        Direction::Up.lookups().filter_map(move |(dx, dy)| {
            let x = x
                .checked_add_signed(dx)
                .and_then(|x| (x < self.width()).then_some(x));

            let y = y
                .checked_add_signed(dy)
                .and_then(|y| (y < self.height()).then_some(y));

            x.and_then(|x| y.map(|y| (x, y)))
        })
    }

    pub fn regions(&self) -> HashMap<(usize, usize), Region> {
        fn inner(
            g: &Garden,
            v: &mut HashSet<(usize, usize)>,
            s: &mut HashMap<(usize, usize), HashSet<Direction>>,
            x: usize,
            y: usize,
            c: char,
        ) -> Region {
            let area = 1;
            let mut perimeter = 4;
            v.insert((x, y));

            let r = g
                .adjacent(x, y)
                .filter_map(|(x, y)| {
                    if g.tiles[y][x] == c {
                        perimeter -= 1;

                        if !v.contains(&(x, y)) {
                            Some(inner(g, v, s, x, y, c))
                        } else {
                            Some(Region {
                                area: 0,
                                perimeter: 0,
                                sides: 0,
                            })
                        }
                    } else {
                        None
                    }
                })
                .reduce(|acc, r| Region {
                    area: acc.area + r.area,
                    perimeter: acc.perimeter + r.perimeter,
                    sides: acc.sides + r.sides,
                })
                .unwrap_or(Region {
                    area: 0,
                    perimeter: 0,
                    sides: 0,
                });

            for (k, v) in Direction::Up.lookups().filter_map(|(dx, dy)| {
                let (nx, ny) = (x as isize + dx, y as isize + dy);

                if nx < 0 {
                    Some(((x, y), Direction::Left))
                } else if ny < 0 {
                    Some(((x, y), Direction::Up))
                } else if nx as usize >= g.width() {
                    Some(((x, y), Direction::Right))
                } else if ny as usize >= g.height() {
                    Some(((x, y), Direction::Down))
                } else if g.tiles[ny as usize][nx as usize] != c {
                    Some((
                        (x, y),
                        Direction::from_vector(x, y, nx as usize, ny as usize),
                    ))
                } else {
                    None
                }
            }) {
                s.entry(k)
                    .and_modify(|h| {
                        h.insert(v);
                    })
                    .or_insert_with(|| {
                        let mut h = HashSet::new();
                        h.insert(v);
                        h
                    });
            }

            Region {
                area: r.area + area,
                perimeter: r.perimeter + perimeter,
                sides: r.sides,
            }
        }

        let mut visited = HashSet::new();

        self.all_coordinates()
            .fold(HashMap::new(), |mut h, (x, y)| {
                if !visited.contains(&(x, y)) {
                    let c = self.tiles[y][x];
                    let mut v = HashSet::new();
                    let mut s = HashMap::new();

                    let mut r = inner(self, &mut v, &mut s, x, y, c);
                    r.sides = calculate_sides(&s);
                    h.insert((x, y), r);
                    visited.extend(v);
                }

                h
            })
    }

    pub fn fence_price(&self) -> usize {
        self.regions()
            .into_values()
            .map(|r| r.area * r.perimeter)
            .sum()
    }

    pub fn fence_price_discount(&self) -> usize {
        self.regions().into_values().map(|r| r.area * r.sides).sum()
    }
}

fn part_1(input: &str) {
    let garden: Garden = input.parse().unwrap();

    let result = garden.fence_price();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let garden: Garden = input.parse().unwrap();

    let result = garden.fence_price_discount();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
