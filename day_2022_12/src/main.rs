use std::{collections::VecDeque, ops::Index};

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

// Let's define a type alias for coordinates, so that it's easier to write
// functions using them.
type Coord = (usize, usize);

// Turns 2D coordinates into a 1D vector index.
fn coord_to_idx((x, y): Coord, width: usize) -> usize {
    y * width + x
}

// Turns a 1D vector index into 2D coordinates.
fn idx_to_coord(idx: usize, width: usize) -> Coord {
    (idx % width, idx / width)
}

/// The direction in which we hike through the mountain.
#[derive(Clone, Copy)]
enum HikingDirection {
    Up,
    Down,
}

/// An elevation map of the terrain.
struct Map {
    /// Width of the map.
    width: usize,
    /// Height of the map.
    height: usize,
    /// Hike start coordinates, as indicated in the input.
    start: Coord,
    /// Hike end coordinates, as indicated in the input.
    end: Coord,
    /// Elevation map stored in a 1D vector.
    elevations: Vec<u8>,
}

impl Map {
    /// Creates a map from the input.
    pub fn new(input: &str) -> Self {
        // The width of the map is the number of characters on a line of the
        // input.
        let width = input.lines().next().unwrap().chars().count();
        // The height of the map is the number of lines in the input.
        let height = input.lines().count();

        // We'll need the start and end positions, but we don't know what they
        // are yet, so leave them as None for the moment.
        let mut start = None;
        let mut end = None;

        // Iterating over the characters of the input...
        let elevations = input
            .chars()
            // ...filter out the ones we're not interested in...
            .filter(|c| matches!(c, 'a'..='z' | 'S' | 'E'))
            // ...enumerate the good characters to get their position...
            .enumerate()
            // ...and for each of them, compute their elevation value:
            .map(|(idx, c)| match c {
                // Lowercase letters are assigned elevation according to their
                // alphabetical order.
                'a'..='z' => c as u8 - b'a',
                // 'S' is the start, at elevation 0.
                'S' => {
                    // We also update the start position.
                    start = Some(idx_to_coord(idx, width));
                    0
                }
                // 'E' is the end, at elevation 25.
                'E' => {
                    // We also update the end position.
                    end = Some(idx_to_coord(idx, width));
                    25
                }
                // We have filtered out any other character beforehand, so it's
                // OK to panic here.
                _ => unreachable!(),
            })
            // Collect those new elevation values into a vector.
            .collect();

        Self {
            width,
            height,
            // If `start` and `end` were None, it would mean the input didn't
            // contain any 'S' or 'E' character and would be invalid, so it's
            // OK to unwrap here.
            start: start.unwrap(),
            end: end.unwrap(),
            elevations,
        }
    }

    /// Returns an iterator over the coordinates that are accessible from a
    /// given set of coordinate, and the direction in which we're hiking.
    fn available_paths(
        &self,
        (x, y): Coord,
        hiking_direction: HikingDirection,
    ) -> impl Iterator<Item = Coord> + '_ {
        // First, enumerate all the theoretically possible directions.
        [
            // East, but only if we're not on the right edge of the map.
            if x + 1 < self.width {
                Some((x + 1, y))
            } else {
                None
            },
            // South, but only if we're not on the bottom edge of the map.
            if y + 1 < self.height {
                Some((x, y + 1))
            } else {
                None
            },
            // West, but only if we're not on the left edge of the map.
            x.checked_sub(1).map(|x| (x, y)),
            // North, but only if we're not on the top edge of the map.
            y.checked_sub(1).map(|y| (x, y)),
        ]
        // Then iterate on these coordinates...
        .into_iter()
        // ...and filter out all the impossible coordinates:
        .filter_map(move |maybe_coord| {
            // We don't want coordinates that would lead us out of bounds.
            maybe_coord.and_then(|(lx, ly)| {
                // We also need to check if the elevation difference is correct,
                // and this depends on the direction we hike:
                let condition = match hiking_direction {
                    // When going up, we can only go at most one elevation up.
                    HikingDirection::Up => self[(lx, ly)] <= self[(x, y)] + 1,
                    // When going down, we can only go at most one elevation down.
                    HikingDirection::Down => self[(lx, ly)] + 1 >= self[(x, y)],
                };

                // If it's safe to hike there, then yield this coordinate;
                // otherwise discard it.
                if condition {
                    Some((lx, ly))
                } else {
                    None
                }
            })
        })
    }

    /// Compute the shortest path from a given starting position and a hike
    /// direction, and yielding the shortest path to any of the given end
    /// positions.
    fn shortest_path<I>(&self, start: Coord, ends: I, hiking_direction: HikingDirection) -> usize
    where
        I: IntoIterator<Item = Coord>,
    {
        // We're going to perform a path search.

        // We are first going to consider the starting point.
        let mut pending: VecDeque<Coord> = [start].into();

        // For now, we can't reach any tile, so initialise the shortest path
        // array to the maximum value of a usize...
        let mut lookup = vec![usize::MAX; self.elevations.len()];
        // but set the value to 0 for the start position, as we're already here.
        lookup[coord_to_idx(start, self.width)] = 0;

        // Now, while there still is a set of coordinates to consider...
        while let Some((x, y)) = pending.pop_front() {
            // Get the current shortest path to that position.
            let current_len = lookup[coord_to_idx((x, y), self.width)];

            // We want to replenish the list of pending paths. We are going to
            // dd to it the available adjacent paths...
            pending.extend(self.available_paths((x, y), hiking_direction).filter_map(
                |(path_x, path_y)| {
                    // ...but only if we can get here faster than we already have
                    // done.

                    // Get the current best for that set of coordinates.
                    let best = &mut lookup[coord_to_idx((path_x, path_y), self.width)];

                    // If we got here even faster than that:
                    if *best > current_len + 1 {
                        // We can update our record...
                        *best = current_len + 1;
                        // ...and add this set of coordinates to the pending
                        // list;
                        Some((path_x, path_y))
                    } else {
                        // Otherwise, we don't need to consider it.
                        None
                    }
                },
            ));
        }

        // After having gone everywhere we could have gone, we need to find the
        // shortest path to any of the ends. So, for each end position...
        ends.into_iter()
            // ...get the length of the path from the start to that position...
            .map(|(x, y)| lookup[coord_to_idx((x, y), self.width)])
            // ...and find the smallest of these values.
            .min()
            // We should always have at least one end position, so we can unwrap
            // here.
            .unwrap()
    }

    /// Finds the shortest path from the starting position to the end position.
    fn shortest_path_to_top(&self) -> usize {
        // To do that, we go from the start, up to the end (and only the end),
        // while going up.
        self.shortest_path(self.start, std::iter::once(self.end), HikingDirection::Up)
    }

    /// Finds the shortest path to any point with elevation 0 on the map.
    fn path_with_shortest_hike(&self) -> usize {
        // To do that, we find the shortest path...
        self.shortest_path(
            // ...from the end point...
            self.end,
            // ...to any of the points...
            self.elevations
                .iter()
                .enumerate()
                // ...whose elevation is 0...
                .filter_map(|(idx, elevation)| {
                    if *elevation == 0 {
                        Some(idx_to_coord(idx, self.width))
                    } else {
                        None
                    }
                }),
            // ...while going down the mountain.
            HikingDirection::Down,
        )
    }
}

// A useful way to index the map using a set of coordinates.
impl Index<Coord> for Map {
    type Output = u8;

    fn index(&self, (x, y): Coord) -> &Self::Output {
        self.elevations.index(coord_to_idx((x, y), self.width))
    }
}

fn part_1(input: &str) {
    // In the first part, we go from the start to the end.

    let map = Map::new(input);

    let result = map.shortest_path_to_top();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // In the second part, we go from the end to any of the ground positions.

    let map = Map::new(input);

    let result = map.path_with_shortest_hike();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
