use std::collections::HashSet;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

/// The 4 directions the head of a rope can travel to.
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Returns the direction vector of this direction.
    fn vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
        }
    }
}

/// The rope of the problem, containing an arbitrary number of knots in
/// addition to the "head" node.
struct Rope {
    /// The list of knots on the rope.
    knots: Vec<(i32, i32)>,
    /// The set of positions the last knot of the rope, the "tail", has been to
    /// at least once.
    tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    /// Initialises a new Rope with the specified amount of knots, not
    /// including the head.
    fn new(knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); knots + 1],
            // The tail always visit the starting position at least once.
            tail_positions: [(0, 0)].into(),
        }
    }

    /// Moves the head of the rope a certain number of times in a specific
    /// direction. This may also move the other knots on the rope.
    fn move_towards(&mut self, direction: Direction, amount: usize) {
        // Firstly, let's get the direction vector of where the rope is going.
        let (dx, dy) = direction.vector();

        // We are going to move `amount` times in the direction.
        for _ in 0..amount {
            // Move the head in the direction.
            self.knots[0].0 += dx;
            self.knots[0].1 += dy;

            // For every knot index there is, excluding the tail:
            for split_idx in 0..self.knots.len() - 1 {
                // Split the list of knots in two, at the current
                // `split_idx` + 1. This makes the current "head" knot the last
                // knot in the left slice, and the current "tail" knot the first
                // one in the second slice. I'm using pattern matching to
                // extract them.
                //
                // This allows us to pretty elegantly get a mutable reference on
                // both the current "head" and current "tail", by exploiting the
                // invariant of `split_at_mut` that the two slices given are
                // disjoint.
                let ([.., (head_x, head_y)], [(tail_x, tail_y), ..]) =
                    self.knots.split_at_mut(split_idx + 1)
                else {
                    // Given the construction of this pattern matching, we
                    // should always have at least one element in each slice
                    // given by `split_at_mut`, so we can panic here.
                    unreachable!()
                };

                // Now, look at the distance on each dimension between the
                // current "head" and "tail":
                match ((*head_x - *tail_x).abs(), (*head_y - *tail_y).abs()) {
                    // They are either stacked on top of each other (the (0, 0)
                    // variant), or adjacent (the other variants). In this case,
                    // we can do nothing, and since we haven't moved, we know
                    // that the knot behind us won't move either, so we can stop
                    // the knot iteration early by breaking.
                    (0, 0) | (0, 1) | (1, 0) | (1, 1) => {
                        break;
                    }
                    // The two knots are not adjacent or stacked, so the "tail"
                    // needs to move.
                    _ => {
                        // The tail has to go towards the head, so in each
                        // direction, compute the sign and add that value to
                        // the tail. This works because in the event the two
                        // knots are aligned in a direction, `signum()` will
                        // return 0.
                        *tail_x += (*head_x - *tail_x).signum();
                        *tail_y += (*head_y - *tail_y).signum();
                    }
                }
            }

            self.tail_positions.insert(*self.knots.last().unwrap());
        }
    }

    /// Returns the number of unique positions of the rope's tail after
    /// moving according to the input.
    fn amount_of_tail_positions(&mut self, moves: &str) -> usize {
        // For each line in the input:
        for line in moves.lines() {
            // Separate the direction and the amount.
            let (direction, amount) = line.split_once(' ').unwrap();

            // Parse the direction according to its letter.
            let direction = match direction {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                // We are guaranteed to have valid input, so we can panic here.
                _ => unreachable!(),
            };

            // Parse the amount. Likewise, since the input has to be valid, we
            // can unwrap here.
            let amount = amount.parse::<usize>().unwrap();

            // Perform the moves.
            self.move_towards(direction, amount);
        }

        // Return the number of unique positions.
        self.tail_positions.len()
    }
}

fn part_1(input: &str) {
    // In this part, we need to find the number of unique positions of the
    // "tail" of a rope, after a number of moves.

    let mut rope = Rope::new(1);

    let result = rope.amount_of_tail_positions(input);

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    // This second part, we have 9 knots instead of 1.

    let mut rope = Rope::new(9);

    let result = rope.amount_of_tail_positions(input);

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
