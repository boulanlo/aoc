use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
enum Axis {
    NS,
    EW,
}

#[derive(Debug)]
enum Mirror {
    Nwse,
    Nesw,
}

#[derive(Debug)]
enum Tile {
    Empty,
    Splitter(Axis),
    Mirror(Mirror),
}

type Beam = ((usize, usize), Direction);

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn go(&self, ((x, y), d): Beam) -> Option<Beam> {
        match d {
            Direction::N => y.checked_sub(1).map(|y| ((x, y), Direction::N)),
            Direction::E => (x < self.width() - 1).then_some(((x + 1, y), Direction::E)),
            Direction::S => (y < self.height() - 1).then_some(((x, y + 1), Direction::S)),
            Direction::W => x.checked_sub(1).map(|x| ((x, y), Direction::W)),
        }
    }

    fn borders(&self) -> impl Iterator<Item = Beam> + '_ {
        (0..self.width())
            .map(|x| ((x, 0), Direction::S))
            .chain((0..self.height()).map(|y| ((0, y), Direction::E)))
            .chain((0..self.width()).map(|x| ((x, self.height() - 1), Direction::N)))
            .chain((0..self.height()).map(|y| ((self.width() - 1, y), Direction::W)))
    }

    pub fn beam(&self, ((x, y), d): Beam) -> usize {
        let mut visited = HashSet::new();
        let mut beams = vec![((x, y), d)];

        while !beams.is_empty() {
            beams = beams
                .into_iter()
                .flat_map(|((x, y), d)| {
                    if visited.contains(&((x, y), d)) {
                        Vec::new()
                    } else {
                        visited.insert(((x, y), d));

                        match (&self.tiles[y][x], d) {
                            (Tile::Empty, d) => vec![self.go(((x, y), d))],
                            (Tile::Splitter(Axis::EW), Direction::E | Direction::W)
                            | (Tile::Splitter(Axis::NS), Direction::N | Direction::S) => {
                                vec![self.go(((x, y), d))]
                            }
                            (Tile::Splitter(Axis::EW), Direction::N | Direction::S) => {
                                vec![
                                    self.go(((x, y), Direction::W)),
                                    self.go(((x, y), Direction::E)),
                                ]
                            }
                            (Tile::Splitter(Axis::NS), Direction::E | Direction::W) => {
                                vec![
                                    self.go(((x, y), Direction::N)),
                                    self.go(((x, y), Direction::S)),
                                ]
                            }
                            (Tile::Mirror(Mirror::Nesw), Direction::E)
                            | (Tile::Mirror(Mirror::Nwse), Direction::W) => {
                                vec![self.go(((x, y), Direction::N))]
                            }
                            (Tile::Mirror(Mirror::Nesw), Direction::W)
                            | (Tile::Mirror(Mirror::Nwse), Direction::E) => {
                                vec![self.go(((x, y), Direction::S))]
                            }
                            (Tile::Mirror(Mirror::Nesw), Direction::N)
                            | (Tile::Mirror(Mirror::Nwse), Direction::S) => {
                                vec![self.go(((x, y), Direction::E))]
                            }
                            (Tile::Mirror(Mirror::Nesw), Direction::S)
                            | (Tile::Mirror(Mirror::Nwse), Direction::N) => {
                                vec![self.go(((x, y), Direction::W))]
                            }
                        }
                    }
                })
                .flatten()
                .collect();
        }

        visited
            .into_iter()
            .map(|(xy, _)| xy)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl<'a> FromIterator<&'a str> for Map {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Map {
            tiles: iter
                .into_iter()
                .map(|s| {
                    s.chars()
                        .map(|c| match c {
                            '.' => Tile::Empty,
                            '/' => Tile::Mirror(Mirror::Nesw),
                            '\\' => Tile::Mirror(Mirror::Nwse),
                            '|' => Tile::Splitter(Axis::NS),
                            '-' => Tile::Splitter(Axis::EW),
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

fn part_1(input: &str) {
    let map = input.lines().collect::<Map>();

    println!("{:?}", map.beam(((0, 0), Direction::E)));
}

fn part_2(input: &str) {
    let map = input.lines().collect::<Map>();

    println!("{:?}", map.borders().map(|b| map.beam(b)).max());
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
