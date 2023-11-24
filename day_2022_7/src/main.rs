use std::iter::Peekable;

use aoc_utils::*;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    size: usize,
    children: Vec<Dir<'a>>,
}

fn parse_filesystem(input: &str) -> Dir {
    fn inner<'a, I: Iterator<Item = &'a str>>(
        input: &mut Peekable<I>,
        mut name: Option<&'a str>,
    ) -> Dir<'a> {
        let mut size = 0;
        let mut children = Vec::new();

        while let Some(l) = input.next() {
            let mut tokens = l.split_ascii_whitespace();
            assert_eq!(tokens.next(), Some("$"));

            match tokens.next() {
                Some("cd") => match tokens.next() {
                    Some("..") => break,
                    Some(dir_name) => {
                        if name.is_some() {
                            let child = inner(input, Some(dir_name));
                            size += child.size;
                            children.push(child);
                        } else {
                            name = Some(dir_name)
                        }
                    }
                    None => unreachable!(),
                },
                Some("ls") => {
                    while let Some(i) = input.peek() {
                        if i.starts_with('$') {
                            break;
                        }
                        let i = input.next().unwrap();
                        let (size_or_dir, _) = i.split_once(' ').unwrap();
                        if size_or_dir != "dir" {
                            size += size_or_dir.parse::<usize>().unwrap();
                        }
                    }
                }
                Some(_) => unreachable!(),
                None => break,
            }
        }

        Dir {
            name: name.unwrap(),
            size,
            children,
        }
    }

    inner(&mut input.lines().peekable(), None)
}

impl<'a> Dir<'a> {
    fn sum_with_threshold(&self, max: usize) -> usize {
        self.children
            .iter()
            .map(|d| d.sum_with_threshold(max))
            .sum::<usize>()
            + if self.size <= max { self.size } else { 0 }
    }

    fn dirs(&self) -> impl Iterator<Item = &Dir> {
        std::iter::once(self).chain(
            self.children
                .iter()
                .flat_map(|d| d.dirs().collect::<Vec<_>>()),
        )
    }
}

fn part_1(input: &str) {
    let dir = parse_filesystem(input);

    println!("{}", dir.sum_with_threshold(100000));
}

fn part_2(input: &str) {
    let dir = parse_filesystem(input);
    let unused = 70000000 - dir.size;
    let required = 30000000 - unused;

    let x = dir
        .dirs()
        .filter_map(|d| {
            if d.size >= required {
                Some(d.size)
            } else {
                None
            }
        })
        .sorted()
        .next()
        .unwrap();

    println!("{x}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
