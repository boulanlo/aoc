use std::collections::HashSet;

// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

const EXAMPLE_2: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

const EXAMPLE_3: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

const EXAMPLE_4: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_coordinates((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Self {
        match (x2 as isize - x1 as isize, y2 as isize - y1 as isize) {
            (1, 0) => Direction::East,
            (-1, 0) => Direction::West,
            (0, 1) => Direction::South,
            (0, -1) => Direction::North,
            _ => unreachable!(),
        }
    }

    fn search_coordinates<'a>(
        &self,
        start_x: usize,
        start_y: usize,
        pipes: &'a Pipes,
        loop_coords: &'a HashSet<(usize, usize)>,
    ) -> Box<dyn Iterator<Item = (usize, usize)> + 'a> {
        match self {
            Direction::North => Box::new(
                (0..start_y)
                    .rev()
                    .map(move |y| (start_x, y))
                    .take_while(|(x, y)| !loop_coords.contains(&(*x, *y))),
            ),
            Direction::East => Box::new(
                (start_x + 1..pipes.width())
                    .map(move |x| (x, start_y))
                    .take_while(|(x, y)| !loop_coords.contains(&(*x, *y))),
            ),
            Direction::South => Box::new(
                (start_y + 1..pipes.height())
                    .map(move |y| (start_x, y))
                    .take_while(|(x, y)| !loop_coords.contains(&(*x, *y))),
            ),
            Direction::West => Box::new(
                (0..start_x)
                    .rev()
                    .map(move |x| (x, start_y))
                    .take_while(|(x, y)| !loop_coords.contains(&(*x, *y))),
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
}

impl Tile {
    fn connects_north(&self) -> bool {
        matches!(self, Tile::NS | Tile::NE | Tile::NW)
    }

    fn connects_south(&self) -> bool {
        matches!(self, Tile::NS | Tile::SW | Tile::SE)
    }

    fn connects_east(&self) -> bool {
        matches!(self, Tile::EW | Tile::NE | Tile::SE)
    }

    fn connects_west(&self) -> bool {
        matches!(self, Tile::EW | Tile::NW | Tile::SW)
    }

    fn connections(&self) -> [(isize, isize); 2] {
        match self {
            Tile::Ground | Tile::Start => unreachable!(),
            Tile::NS => [(0, 1), (0, -1)],
            Tile::EW => [(1, 0), (-1, 0)],
            Tile::NE => [(0, -1), (1, 0)],
            Tile::NW => [(0, -1), (-1, 0)],
            Tile::SW => [(0, 1), (-1, 0)],
            Tile::SE => [(0, 1), (1, 0)],
        }
    }

    fn new_direction(&self, next: &Self) -> Option<Direction> {
        match (self, next) {
            (Tile::NE | Tile::NW, Tile::NS) => Some(Direction::North),
            (_, _) => None,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Ground,
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            'S' => Tile::Start,
            _ => unreachable!(),
        }
    }
}

struct Pipes {
    tiles: Vec<Vec<Tile>>,
}

impl Pipes {
    fn start(&self) -> (usize, usize) {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(y, v)| {
                v.iter().enumerate().find_map(|(x, t)| {
                    if matches!(t, Tile::Start) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn loop_coordinates(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + '_ {
        let (start_x, start_y) = self.start();

        let (north, east, south, west) = (
            start_y
                .checked_sub(1)
                .map(|y| self[(start_x, y)].connects_south())
                .unwrap_or(false),
            if start_x == self.width() {
                false
            } else {
                self[(start_x + 1, start_y)].connects_west()
            },
            if start_y == self.height() {
                false
            } else {
                self[(start_x, start_y + 1)].connects_north()
            },
            start_x
                .checked_sub(1)
                .map(|x| self[(x, start_y)].connects_east())
                .unwrap_or(false),
        );

        let start = match (north, east, south, west) {
            (true, false, true, false) => Tile::NS,
            (false, true, false, true) => Tile::EW,
            (true, true, false, false) => Tile::NE,
            (true, false, false, true) => Tile::NW,
            (false, false, true, true) => Tile::SW,
            (false, true, true, false) => Tile::SE,
            _ => unreachable!(),
        };

        let (next_dx, next_dy) = start.connections()[0];
        let (next_x, next_y) = (
            start_x.checked_add_signed(next_dx).unwrap(),
            start_y.checked_add_signed(next_dy).unwrap(),
        );

        std::iter::successors(
            Some(((start_x, start_y), (next_x, next_y))),
            |((previous_x, previous_y), (x, y))| {
                let (next_x, next_y) = self[(*x, *y)]
                    .connections()
                    .into_iter()
                    .find_map(|(dx, dy)| {
                        let (x, y) = (
                            x.checked_add_signed(dx).unwrap(),
                            y.checked_add_signed(dy).unwrap(),
                        );
                        if (x, y) == (*previous_x, *previous_y) {
                            None
                        } else {
                            Some((x, y))
                        }
                    })
                    .unwrap();

                if matches!(self[(next_x, next_y)], Tile::Start) {
                    None
                } else {
                    Some(((*x, *y), (next_x, next_y)))
                }
            },
        )
    }

    fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width())
            .map(|x| (x, 0))
            .chain((0..self.height()).map(|y| (0, y)))
            .chain((0..self.width()).rev().map(|x| (x, self.height() - 1)))
            .chain((0..self.height()).rev().map(|y| (self.width() - 1, y)))
    }

    fn area_inside(&self) -> usize {
        let loop_coords = self
            .loop_coordinates()
            .flat_map(|(a, b)| vec![a, b])
            .collect::<HashSet<_>>();

        let a =
            self.loop_coordinates()
                .fold(HashSet::new(), |mut h, ((x, y), (next_x, next_y))| {
                    let direction = Direction::from_coordinates((x, y), (next_x, next_y));

                    let search_directions = match (direction, self[(next_x, next_y)]) {
                        (Direction::East, Tile::EW) => vec![Direction::North],
                        (Direction::East, Tile::SW) => vec![Direction::North, Direction::East],
                        (Direction::West, Tile::EW) => vec![Direction::South],
                        (Direction::West, Tile::NE) => vec![Direction::South, Direction::West],
                        (Direction::North, Tile::NS) => vec![Direction::West],
                        (Direction::North, Tile::SE) => vec![Direction::West, Direction::North],
                        (Direction::South, Tile::NS) => vec![Direction::East],
                        (Direction::South, Tile::NW) => vec![Direction::East, Direction::South],
                        _ => vec![],
                    };

                    h.extend(
                        search_directions
                            .into_iter()
                            .flat_map(|d| d.search_coordinates(next_x, next_y, self, &loop_coords)),
                    );

                    h
                });

        let all = (0..self.width())
            .flat_map(|x| (0..self.height()).map(move |y| (x, y)))
            .filter(|(x, y)| !loop_coords.contains(&(*x, *y)))
            .collect::<HashSet<_>>();

        let b = all.difference(&a).copied().collect::<HashSet<_>>();

        let res = self
            .edges()
            .find_map(|(x, y)| {
                if a.contains(&(x, y)) {
                    Some(&b)
                } else if b.contains(&(x, y)) {
                    Some(&a)
                } else {
                    None
                }
            })
            .unwrap();

        res.len()
    }
}

impl<'a> FromIterator<&'a str> for Pipes {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self {
            tiles: iter
                .into_iter()
                .map(|l| l.chars().map(Into::into).collect())
                .collect(),
        }
    }
}

impl std::ops::Index<(usize, usize)> for Pipes {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[y][x]
    }
}

fn part_1(input: &str) {
    let pipes = input.lines().collect::<Pipes>();

    println!("{}", (pipes.loop_coordinates().count() + 1) / 2);
}

fn part_2(input: &str) {
    let pipes = input.lines().collect::<Pipes>();

    println!("{}", pipes.area_inside());
}

fn main() {
    let _input = EXAMPLE;
    part_1(EXAMPLE);
    part_2(EXAMPLE_4);
}
