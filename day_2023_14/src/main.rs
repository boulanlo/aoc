use std::{
    collections::VecDeque,
    fmt,
    ops::{Index, IndexMut},
};

// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Element {
    Ground,
    Cube,
    Boulder,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '.' => Element::Ground,
            'O' => Element::Boulder,
            '#' => Element::Cube,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Boulder => "O",
                Element::Cube => "#",
                Element::Ground => ".",
            }
        )
    }
}

fn compute_boulder_moves<I>(iter: I) -> Vec<((usize, usize), (usize, usize))>
where
    I: IntoIterator<Item = ((usize, usize), Element)>,
{
    iter.into_iter()
        .fold(
            (vec![], VecDeque::new()),
            |(mut v, mut available_spots), (i, e)| {
                match e {
                    Element::Ground => available_spots.push_back(i),
                    Element::Cube => available_spots.clear(),
                    Element::Boulder => {
                        if let Some(spot) = available_spots.pop_front() {
                            available_spots.push_back(i);
                            v.push((i, spot));
                        }
                    }
                }

                (v, available_spots)
            },
        )
        .0
}

#[derive(Clone, PartialEq, Eq)]
struct Platform {
    elements: Vec<Vec<Element>>,
}

impl Platform {
    fn height(&self) -> usize {
        self.elements.len()
    }

    fn width(&self) -> usize {
        self.elements[0].len()
    }

    fn rows_from_west(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), Element)> + '_> + '_ {
        (0..self.height()).map(move |y| (0..self.width()).map(move |x| ((x, y), self[(x, y)])))
    }

    fn rows_from_east(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), Element)> + '_> + '_ {
        (0..self.height())
            .map(move |y| (0..self.width()).rev().map(move |x| ((x, y), self[(x, y)])))
    }

    fn columns_from_north(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), Element)> + '_> + '_ {
        (0..self.width()).map(move |x| (0..self.height()).map(move |y| ((x, y), self[(x, y)])))
    }

    fn columns_from_south(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), Element)> + '_> + '_ {
        (0..self.width()).map(move |x| {
            (0..self.height())
                .rev()
                .map(move |y| ((x, y), self[(x, y)]))
        })
    }

    fn tilt<I, J>(&self, iter: I) -> Self
    where
        I: Iterator<Item = J>,
        J: Iterator<Item = ((usize, usize), Element)>,
    {
        let mut clone = self.clone();

        for (from, to) in iter.flat_map(compute_boulder_moves) {
            clone[from] = Element::Ground;
            clone[to] = Element::Boulder;
        }

        clone
    }

    fn cycle(&mut self) {
        let n = self.tilt(self.columns_from_north());
        *self = n;

        let w = self.tilt(self.rows_from_west());
        *self = w;

        let s = self.tilt(self.columns_from_south());
        *self = s;

        let e = self.tilt(self.rows_from_east());
        *self = e;
    }

    fn current_load(&self) -> usize {
        self.rows_from_west() // direction doesn't matter
            .enumerate()
            .map(|(i, row)| {
                let weight = self.height() - i;

                row.filter(|(_, e)| matches!(e, Element::Boulder)).count() * weight
            })
            .sum::<usize>()
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.elements.iter().try_for_each(|v| {
            v.iter().try_for_each(|e| e.fmt(f))?;
            writeln!(f)
        })
    }
}

impl Index<(usize, usize)> for Platform {
    type Output = Element;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.elements[y][x]
    }
}

impl IndexMut<(usize, usize)> for Platform {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[y][x]
    }
}

impl<'a> FromIterator<&'a str> for Platform {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self {
            elements: iter
                .into_iter()
                .map(|s| s.chars().map(Into::into).collect())
                .collect(),
        }
    }
}

fn part_1(input: &str) {
    let mut platform: Platform = input.lines().collect();

    platform = platform.tilt(platform.columns_from_north());

    let result = platform.current_load();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    const ITERATIONS: usize = 1_000_000_000;

    let mut platform: Platform = input.lines().collect();

    let mut history = Vec::new();
    let mut current_loop = Vec::new();

    for _ in 0..ITERATIONS {
        history.push(platform.clone());
        platform.cycle();

        if let Some(i) = history.iter().position(|p| p == &platform) {
            if current_loop.first() == Some(&i) {
                break;
            } else {
                current_loop.push(i);
            }
        }
    }

    assert!(!current_loop.is_empty());

    let result =
        history[current_loop[(ITERATIONS - current_loop[0]) % current_loop.len()]].current_load();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
