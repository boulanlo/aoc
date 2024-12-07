use std::{collections::HashSet, convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

fn line<const N: usize>(y: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..width).map(move |x| (x, y))
}

fn column<const N: usize>(x: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).map(move |y| (x, y))
}

fn asc_diagonal<const N: usize>(
    i: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let z = N - 1;
    let x = (i + z).min(width - 1);
    let y = i.saturating_sub(width - z - 1);

    std::iter::successors(Some((x, y)), move |(x, y)| {
        x.checked_sub(1).and_then(|x| {
            if *y == height - 1 {
                None
            } else {
                Some((x, y + 1))
            }
        })
    })
}

fn desc_diagonal<const N: usize>(
    i: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let z = N - 1;
    let x = i.saturating_sub(height - z - 1);
    let y = (height - z - 1).saturating_sub(i);

    std::iter::successors(Some((x, y)), move |(x, y)| {
        if *x == width - 1 || *y == height - 1 {
            None
        } else {
            Some((x + 1, y + 1))
        }
    })
}

struct WordSearch {
    letters: Vec<Vec<char>>,
}

impl FromStr for WordSearch {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WordSearch {
            letters: s.lines().map(|l| l.chars().collect()).collect(),
        })
    }
}

impl WordSearch {
    fn width(&self) -> usize {
        self.letters[0].len()
    }

    fn height(&self) -> usize {
        self.letters.len()
    }

    fn lines<const N: usize>(
        &self,
    ) -> impl Iterator<Item = Box<dyn Iterator<Item = (usize, usize)>>> + '_ {
        (0..self.height()).map(|y| Box::new(line::<N>(y, self.width())) as _)
    }

    fn columns<const N: usize>(
        &self,
    ) -> impl Iterator<Item = Box<dyn Iterator<Item = (usize, usize)>>> + '_ {
        (0..self.width()).map(|x| Box::new(column::<N>(x, self.height())) as _)
    }

    fn asc_diagonals<const N: usize>(
        &self,
    ) -> impl Iterator<Item = Box<dyn Iterator<Item = (usize, usize)>>> + '_ {
        (0..self.width() + self.height() - (2 * (N - 1)) - 1)
            .map(|i| Box::new(asc_diagonal::<N>(i, self.width(), self.height())) as _)
    }

    fn desc_diagonals<const N: usize>(
        &self,
    ) -> impl Iterator<Item = Box<dyn Iterator<Item = (usize, usize)>>> + '_ {
        (0..self.width() + self.height() - (2 * (N - 1)) - 1)
            .map(|i| Box::new(desc_diagonal::<N>(i, self.width(), self.height())) as _)
    }

    fn collect_window<const N: usize, I>(&self, i: I) -> Vec<[char; N]>
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        i.into_iter()
            .map(|(x, y)| self.letters[y][x])
            .collect::<Vec<_>>()
            .windows(N)
            .map(|w| w.to_vec().try_into().unwrap())
            .collect::<Vec<_>>()
    }

    fn collect_window_coords<const N: usize, I>(&self, i: I) -> Vec<([char; N], (usize, usize))>
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        assert!(N % 2 == 1, "only works on odd N");

        i.into_iter()
            .map(|(x, y)| (self.letters[y][x], (x, y)))
            .collect::<Vec<_>>()
            .windows(N)
            .map(|w| {
                let (w, c): (Vec<_>, Vec<_>) = w.iter().copied().unzip();
                (w.try_into().unwrap(), c[N / 2])
            })
            .collect::<Vec<_>>()
    }

    fn windows<const N: usize>(&self) -> impl Iterator<Item = [char; N]> + '_ {
        self.lines::<N>()
            .flat_map(|i| self.collect_window(i))
            .chain(self.columns::<N>().flat_map(|i| self.collect_window(i)))
            .chain(
                self.asc_diagonals::<N>()
                    .flat_map(|i| self.collect_window(i)),
            )
            .chain(
                self.desc_diagonals::<N>()
                    .flat_map(|i| self.collect_window(i)),
            )

        // self.desc_diagonals::<N>()
        //     .flat_map(|i| self.collect_window(i))
    }

    fn crosses<const N: usize>(&self, a: [char; N], b: [char; N]) -> usize {
        let x = self
            .asc_diagonals::<N>()
            .flat_map(|i| self.collect_window_coords(i))
            .filter_map(|(w, c)| if w == a || w == b { Some(c) } else { None })
            .collect::<HashSet<_>>();

        let y = self
            .desc_diagonals::<N>()
            .flat_map(|i| self.collect_window_coords(i))
            .filter_map(|(w, c)| if w == a || w == b { Some(c) } else { None })
            .collect::<HashSet<_>>();

        x.intersection(&y).count()
    }
}

fn part_1(input: &str) {
    let word_search: WordSearch = input.parse().unwrap();

    let result = word_search
        .windows::<4>()
        .filter(|a| a == &['X', 'M', 'A', 'S'] || a == &['S', 'A', 'M', 'X'])
        .count();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let word_search: WordSearch = input.parse().unwrap();

    let result = word_search.crosses(['M', 'A', 'S'], ['S', 'A', 'M']);

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
