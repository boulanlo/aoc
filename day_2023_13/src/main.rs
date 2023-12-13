use itertools::{EitherOrBoth, Itertools};

// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Element {
    Rock,
    Ash,
}

impl Element {
    fn change(&mut self) {
        *self = match self {
            Element::Rock => Element::Ash,
            Element::Ash => Element::Rock,
        }
    }
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => unreachable!(),
        }
    }
}

fn is_reflection<I>(mut iter: I) -> bool
where
    I: Iterator<Item = EitherOrBoth<Vec<Element>, Vec<Element>>>,
{
    iter.all(|e| match e {
        EitherOrBoth::Both(l, r) => l == r,
        EitherOrBoth::Left(_) | EitherOrBoth::Right(_) => true,
    })
}

#[derive(Clone)]
struct Pattern {
    tiles: Vec<Vec<Element>>,
}

impl Pattern {
    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn vertical_reflections(
        &self,
        start_right: usize,
    ) -> impl Iterator<Item = EitherOrBoth<Vec<Element>, Vec<Element>>> + '_ {
        let width = self.width();
        let height = self.height();

        (0..start_right)
            .rev()
            .map(move |x| (0..height).map(move |y| self[(x, y)]).collect())
            .zip_longest(
                (start_right..width).map(move |x| (0..height).map(|y| self[(x, y)]).collect()),
            )
    }

    fn horizontal_reflections(
        &self,
        start_bottom: usize,
    ) -> impl Iterator<Item = EitherOrBoth<Vec<Element>, Vec<Element>>> + '_ {
        let width = self.width();
        let height = self.height();

        (0..start_bottom)
            .rev()
            .map(move |y| (0..width).map(move |x| self[(x, y)]).collect())
            .zip_longest(
                (start_bottom..height).map(move |y| (0..width).map(|x| self[(x, y)]).collect()),
            )
    }

    fn reflection_scores(&self) -> impl Iterator<Item = usize> + '_ {
        (1..self.width())
            .filter(|v| is_reflection(self.vertical_reflections(*v)))
            .chain(
                (1..self.height()).filter_map(|v| {
                    is_reflection(self.horizontal_reflections(v)).then_some(100 * v)
                }),
            )
    }

    fn variations(&self) -> impl Iterator<Item = Self> + '_ {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .map(|(x, y)| {
                let mut clone = self.clone();
                clone[(x, y)].change();
                clone
            })
    }

    fn reflection_score_with_variations(&self) -> usize {
        let current_score = self.reflection_scores().next().unwrap();

        self.variations()
            .flat_map(|p| p.reflection_scores().collect::<Vec<_>>())
            .find(|s| *s != current_score)
            .unwrap()
    }
}

impl std::ops::Index<(usize, usize)> for Pattern {
    type Output = Element;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[y][x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Pattern {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[y][x]
    }
}

impl<'a> FromIterator<&'a str> for Pattern {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self {
            tiles: iter
                .into_iter()
                .map(|l| l.chars().map(Into::into).collect())
                .collect(),
        }
    }
}

fn part_1(input: &str) {
    let score = input
        .split("\n\n")
        .map(|pattern| {
            let scores = pattern
                .lines()
                .collect::<Pattern>()
                .reflection_scores()
                .collect::<Vec<_>>();
            assert_eq!(scores.len(), 1);
            scores[0]
        })
        .sum::<usize>();

    println!("Part 1: {score}");
}

fn part_2(input: &str) {
    let score = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .collect::<Pattern>()
                .reflection_score_with_variations()
        })
        .sum::<usize>();

    println!("Part 2: {score}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
