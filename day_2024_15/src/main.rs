use std::{collections::HashSet, convert::Infallible, hash::Hash, str::FromStr};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

trait Coordinate: PartialEq + Eq + Hash + Clone + Copy + std::fmt::Debug {
    fn new(x: usize, y: usize) -> Self;
    fn set_x(&mut self, x: usize);
    fn set_y(&mut self, y: usize);
    fn set_side(&mut self, side: bool);
    fn coords(&self) -> (usize, usize, bool);
    fn lateral_advance(&self) -> usize;
    fn right_side(&self) -> Self;

    fn crate_collision(&self, d: Direction) -> HashSet<Self> {
        if d.is_vertical() {
            let o = self.right_side();
            [d.advance(*self), d.advance(o)].into_iter().collect()
        } else {
            let mut c = *self;
            let mut h = HashSet::new();
            if let Direction::Left = d {
                h.insert(d.advance(c));
                h
            } else {
                for _ in 0..3 - self.lateral_advance() {
                    c = d.advance(c);
                }
                h.insert(c);
                h
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct NarrowCoordinate {
    x: usize,
    y: usize,
}

impl Coordinate for NarrowCoordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn coords(&self) -> (usize, usize, bool) {
        (self.x, self.y, false)
    }

    fn lateral_advance(&self) -> usize {
        2
    }

    fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    fn set_side(&mut self, _: bool) {}

    fn right_side(&self) -> Self {
        *self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct WideCoordinate {
    x: usize,
    y: usize,
    right: bool,
}

impl Coordinate for WideCoordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y, right: false }
    }

    fn coords(&self) -> (usize, usize, bool) {
        (self.x, self.y, self.right)
    }

    fn lateral_advance(&self) -> usize {
        1
    }

    fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    fn set_side(&mut self, side: bool) {
        self.right = side;
    }

    fn right_side(&self) -> Self {
        Direction::Right.advance(*self)
    }
}

#[derive(Debug)]
struct Map<C: Coordinate> {
    walls: HashSet<(usize, usize)>,
    crates: HashSet<C>,
    start: C,
}

impl<C: Coordinate> FromStr for Map<C> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut crates = HashSet::new();
        let mut start = None;

        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.char_indices() {
                match c {
                    '#' => {
                        walls.insert((x, y));
                    }
                    'O' => {
                        crates.insert(C::new(x, y));
                    }
                    '@' => {
                        start = Some(C::new(x, y));
                    }
                    _ => {}
                }
            }
        }

        Ok(Self {
            walls,
            crates,
            start: start.unwrap(),
        })
    }
}

impl<C: Coordinate> Map<C> {
    pub fn run<I>(&mut self, i: I)
    where
        I: IntoIterator<Item = Direction>,
    {
        let mut current = self.start;
        for d in i.into_iter() {
            let next = d.advance(current);

            if self.has_wall(next) {
            } else if let Some(c) = self.has_crate(next) {
                let mut crates = vec![];
                let mut h = HashSet::new();
                h.insert(c);
                crates.push(h);

                'outer: loop {
                    let h = crates
                        .last()
                        .unwrap()
                        .iter()
                        .map(|c| c.crate_collision(d))
                        .reduce(|mut a, b| {
                            a.extend(b);
                            a
                        })
                        .unwrap();

                    let mut crate_set = HashSet::new();
                    let mut empty = 0usize;

                    for c in &h {
                        if self.has_wall(*c) {
                            break 'outer;
                        } else if let Some(b) = self.has_crate(*c) {
                            crate_set.insert(b);
                        } else {
                            empty += 1;
                        }
                    }

                    if empty == h.len() {
                        let crates = crates
                            .into_iter()
                            .flat_map(|h| h.into_iter())
                            .collect::<HashSet<_>>();
                        for c in &crates {
                            self.crates.remove(c);
                        }
                        for c in crates {
                            self.crates.insert(d.advance(c));
                        }
                        current = next;
                        break;
                    } else {
                        crates.push(crate_set);
                    }
                }
            } else {
                current = next;
            }
        }
    }

    fn has_wall(&self, c: C) -> bool {
        let (cx, cy, _) = c.coords();
        self.walls.iter().any(|(x, y)| *x == cx && *y == cy)
    }

    fn has_crate(&self, c: C) -> Option<C> {
        let (cx, cy, cs) = c.coords();

        self.crates
            .iter()
            .find(|o| {
                let (ox, oy, os) = o.coords();
                if cs {
                    cx == ox && cy == oy
                } else {
                    (cx.checked_sub(1).map(|cx| ox == cx).unwrap_or(false) || !os)
                        && (os || cx == ox)
                        && cy == oy
                }
            })
            .copied()
    }

    fn gps(&self) -> usize {
        self.crates
            .iter()
            .map(|c| {
                let (x, y, right) = c.coords();

                y * 100 + (x * (3 - c.lateral_advance()) + right as usize)
            })
            .sum()
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    pub fn advance<C: Coordinate>(&self, mut c: C) -> C {
        let (mut x, mut y, mut side) = c.coords();

        let n = if self.is_vertical() {
            1
        } else {
            c.lateral_advance()
        };

        for _ in 0..n {
            match (self, side) {
                (Direction::Up, _) => c.set_y(y - 1),
                (Direction::Down, _) => c.set_y(y + 1),

                (Direction::Right, false) => {
                    c.set_side(true);
                    side = true;
                }
                (Direction::Left, true) => {
                    c.set_side(false);
                    side = false;
                }
                (Direction::Right, true) => {
                    c.set_x(x + 1);
                    c.set_side(false);
                    side = false;
                }
                (Direction::Left, false) => {
                    c.set_x(x - 1);
                    c.set_side(true);
                    side = true;
                }
            }

            (x, y, _) = c.coords();
        }

        c
    }
}

fn part_1(input: &str) {
    let (map, instrs) = input.split_once("\n\n").unwrap();

    let mut map: Map<NarrowCoordinate> = map.parse().unwrap();

    map.run(instrs.lines().flat_map(|l| l.chars()).map(Direction::from));

    let result = map.gps();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let (map, instrs) = input.split_once("\n\n").unwrap();

    let mut map: Map<WideCoordinate> = map.parse().unwrap();

    map.run(instrs.lines().flat_map(|l| l.chars()).map(Direction::from));

    let result = map.gps();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
