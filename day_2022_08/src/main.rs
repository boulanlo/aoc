use std::ops::Index;

use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"30373
25512
65332
33549
35390
"#;

/// A map of the terrain containing trees.
#[derive(Debug)]
struct Map {
    /// The width of the map, in trees.
    width: usize,
    /// The height of the map, in trees.
    height: usize,
    /// The list of trees.
    trees: Vec<u8>,
}

impl Map {
    /// Create a map from the problem's input.
    fn new(input: &str) -> Self {
        // The width of the map can be easily derived from the number of
        // characters in one line of the input.
        let width = input.lines().next().unwrap().chars().count();
        // The height of the map is the number of newlines in the input.
        let height = input.chars().filter(|c| *c == '\n').count();

        Self {
            width,
            height,
            // The list of trees is the input...
            trees: input
                .chars()
                // ...using only characters that are ASCII digits,
                // converted to integers.
                .filter_map(|c| {
                    if c.is_ascii_digit() {
                        Some((c as u8) - b'0')
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    /// Returns an iterator containing 4 inner iterators, each returning the
    /// positions of the trees in each cardinal direction, visible from a given
    /// position (not factoring in the height of the trees).
    fn lookups(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = Box<dyn Iterator<Item = (usize, usize)>>> {
        [
            // North: all y positions from the original y minus 1 to 0.
            Box::new((0..y).rev().map(move |y| (x, y))),
            // East: all y positions from the original y plus 1 to the end
            // at the right.
            Box::new(((x + 1).min(self.width)..self.width).map(move |x| (x, y)))
                as Box<dyn Iterator<Item = (usize, usize)>>,
            // South: all x positions from the original x plus 1 to the end at
            // the bottom.
            Box::new(((y + 1).min(self.height)..self.height).map(move |y| (x, y)))
                as Box<dyn Iterator<Item = (usize, usize)>>,
            // West: all y positions from the original x minus 1 to 0.
            Box::new((0..x).rev().map(move |x| (x, y))) as Box<dyn Iterator<Item = (usize, usize)>>,
        ]
        .into_iter()
    }

    /// Returns the number of visible trees from the edges of the map.
    fn visible(&self) -> usize {
        // For all possible tree indices...
        (0..self.trees.len())
            // ...filter them:
            .filter_map(|i| {
                // Compute the x and y coordinate of the tree.
                let (x, y) = (i % self.width, i / self.width);
                // Get the value of that tree.
                let tree = self[(x, y)];

                // Now: considering all trees in the 4 cardinal directions of
                // that tree...
                if self
                    .lookups((x, y))
                    // ...is there any direct in which all the trees are shorter
                    // that this one, making it visible from the outside of the
                    // map?
                    .any(|mut i| i.all(|(x, y)| self[(x, y)] < tree))
                {
                    Some(())
                } else {
                    None
                }
            })
            // And finally, count how many of the trees on this map are visible
            // from the outside.
            .count()
    }

    /// Computes the maximum "scenic score" achieved on this map.
    fn max_scenic_score(&self) -> usize {
        // For each of the trees on the map:
        (0..self.trees.len())
            .map(|i| {
                // Get its position.
                let (x, y) = (i % self.width, i / self.width);
                // And get the tree's height.
                let tree = self[(x, y)];

                // For each tree in each of the cardinal directions from this
                // current tree...
                self.lookups((x, y))
                    // ...compute the scenic score:
                    .map(|mut i| {
                        // Take all of the trees in that cardinal direction...
                        i.take_while_inclusive(|(x, y)| {
                            // ...and retain only those that are smaller than the
                            // current tree...
                            self[(*x, *y)] < tree
                        })
                        // ...and count them.
                        .count()
                    })
                    // Multiply everything to get the final scenic score of
                    // that tree.
                    .product()
            })
            // Find the maximum achieved scenic score and return that.
            .max()
            // We can unwrap here because there is always going to be at least
            // one tree on the map.
            .unwrap()
    }
}

// Allow indexing the map for the tree heights using a x and y coordinate system.
impl Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let idx = y * self.width + x;
        self.trees.index(idx)
    }
}

fn part_1(input: &str) {
    // In the first part, we need to count how many trees are visible from the
    // edge of the map.

    let map = Map::new(input);
    println!("Part 1: {}", map.visible());
}

fn part_2(input: &str) {
    // In the second part, we want the highest scenic score on the map.

    let map = Map::new(input);
    println!("Part 2: {}", map.max_scenic_score());
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
