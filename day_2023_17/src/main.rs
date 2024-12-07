use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn go(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::N => (x, y - 1),
            Direction::E => (x + 1, y),
            Direction::S => (x, y + 1),
            Direction::W => (x - 1, y),
        }
    }

    pub fn all_set() -> HashSet<Self> {
        [Direction::N, Direction::S, Direction::E, Direction::W]
            .into_iter()
            .collect()
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }

    pub fn turns(&self) -> [Direction; 2] {
        match self {
            Direction::N | Direction::S => [Direction::E, Direction::W],
            Direction::E | Direction::W => [Direction::N, Direction::S],
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct TileParameters {
    position: (usize, usize),
    straight_walk: usize,
    direction: Direction,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct VisitedTile {
    tile: TileParameters,
    heat_loss: usize,
}

impl Ord for VisitedTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for VisitedTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<usize>>,
}

impl<'a> FromIterator<&'a str> for Map {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Map {
            tiles: iter
                .into_iter()
                .map(|l| l.chars().map(|c| (c as u8 - b'0') as usize).collect())
                .collect(),
        }
    }
}

impl Map {
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn find_path<F: Fn(&TileParameters) -> HashSet<Direction>>(
        &self,
        crucible_rule: F,
    ) -> usize {
        let mut stack = BinaryHeap::new();
        stack.push(VisitedTile {
            tile: TileParameters {
                position: (0, 0),
                straight_walk: 0,
                direction: Direction::E,
            },
            heat_loss: 0,
        });

        let mut goal = None;
        let mut dist = HashMap::new();
        dist.insert(
            TileParameters {
                position: (0, 0),
                straight_walk: 0,
                direction: Direction::E,
            },
            0,
        );

        while let Some(current) = stack.pop() {
            // println!("{current:?}");

            if current.tile.position == (self.width() - 1, self.height() - 1) {
                goal = Some(current.heat_loss);
                break;
            } else if dist.get(&current.tile).unwrap_or(&usize::MAX) <= &current.heat_loss {
                let dirs = crucible_rule(&current.tile);

                for t in dirs.into_iter().map(|d| {
                    let position = d.go(current.tile.position);
                    VisitedTile {
                        tile: TileParameters {
                            position,
                            straight_walk: if d == current.tile.direction {
                                current.tile.straight_walk + 1
                            } else {
                                1
                            },
                            direction: d,
                        },
                        heat_loss: current.heat_loss + self.tiles[position.1][position.0],
                    }
                }) {
                    if &t.heat_loss < dist.get(&t.tile).unwrap_or(&usize::MAX) {
                        // println!("  -> {t:?}");
                        dist.insert(t.tile, t.heat_loss);
                        stack.push(t);
                    }
                }
            }
        }

        goal.unwrap()
    }
}

fn part_1(input: &str) {
    let map = input.lines().collect::<Map>();

    println!(
        "{}",
        map.find_path(|t| {
            let mut dirs = Direction::all_set();

            if t.straight_walk == 3 {
                dirs.remove(&t.direction);
            }

            dirs.remove(&t.direction.opposite());

            if t.position.0 == 0 {
                dirs.remove(&Direction::W);
            }
            if t.position.0 == map.width() - 1 {
                dirs.remove(&Direction::E);
            }
            if t.position.1 == 0 {
                dirs.remove(&Direction::N);
            }
            if t.position.1 == map.height() - 1 {
                dirs.remove(&Direction::S);
            }

            dirs
        })
    )
}

fn part_2(input: &str) {
    let map = input.lines().collect::<Map>();

    println!(
        "{}",
        map.find_path(|t| {
            let mut dirs = Direction::all_set();

            if t.straight_walk < 4 {
                for d in t.direction.turns() {
                    dirs.remove(&d);
                }
            }

            dirs.remove(&t.direction.opposite());

            if t.straight_walk == 10 {
                dirs.remove(&t.direction);
            }

            if t.position.0 == 0 {
                dirs.remove(&Direction::W);
            }
            if t.position.0 == map.width() - 1 {
                dirs.remove(&Direction::E);
            }
            if t.position.1 == 0 {
                dirs.remove(&Direction::N);
            }
            if t.position.1 == map.height() - 1 {
                dirs.remove(&Direction::S);
            }

            dirs
        })
    )
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
