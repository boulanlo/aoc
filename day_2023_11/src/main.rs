use aoc_utils::*;
use itertools::Itertools;

// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

fn manhattan_distance(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize))) -> usize {
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

#[derive(Debug)]
struct Universe<const AGE: usize> {
    galaxies: Vec<(usize, usize)>,
}

impl<const AGE: usize> Universe<AGE> {
    fn pairs(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + '_ {
        self.galaxies.iter().copied().tuple_combinations()
    }

    fn sum_of_distances(&self) -> usize {
        self.pairs().map(manhattan_distance).sum::<usize>()
    }
}

impl<'a, const AGE: usize> FromIterator<&'a str> for Universe<AGE> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let expand = |(x, y): (usize, usize), lines: &[Vec<bool>]| {
            let expanded_columns = (0..x).filter(|x| lines.iter().all(|l| l[*x])).count();
            let expanded_lines = lines
                .iter()
                .take(y)
                .filter(|l| l.iter().all(|b| *b))
                .count();

            (
                x + ((AGE - 1) * expanded_columns),
                y + ((AGE - 1) * expanded_lines),
            )
        };

        let lines = iter
            .into_iter()
            .flat_map(|line| {
                let v = line
                    .chars()
                    .map(|c| match c {
                        '#' => false,
                        '.' => true,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();
                vec![v]
            })
            .collect::<Vec<_>>();

        Universe {
            galaxies: lines
                .iter()
                .enumerate()
                .flat_map(|(y, v)| {
                    v.iter()
                        .enumerate()
                        .filter_map(move |(x, b)| if !b { Some((x, y)) } else { None })
                })
                .map(|coord| expand(coord, &lines))
                .collect(),
        }
    }
}

fn part_1(input: &str) {
    let universe: Universe<2> = input.lines().collect();

    let result = universe.sum_of_distances();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let universe: Universe<1_000_000> = input.lines().collect();

    let result = universe.sum_of_distances();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
