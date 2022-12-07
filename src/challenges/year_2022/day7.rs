use std::io::Write;

use color_eyre::Result;
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

const TOTAL_SIZE: u32 = 70000000;
const REQUIRED_UNUSED_SPACE: u32 = 30000000;

#[derive(Debug)]
enum Node {
    File(String, u32),
    Directory(Directory),
}

#[derive(Debug)]
struct Directory {
    name: String,
    contents: Vec<Node>,
    size: u32,
}

impl Directory {
    pub fn fill<I, S>(&mut self, iter: &mut I)
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
    {
        while let Some(line) = iter.next() {
            let mut split = line.as_ref().split_ascii_whitespace();

            match split.next() {
                None => break,
                Some("$") => match split.next() {
                    Some("cd") => {
                        let direction = split.next().unwrap();
                        if direction == ".." {
                            break;
                        } else {
                            let dir = self
                                .contents
                                .iter_mut()
                                .find_map(|d| match d {
                                    Node::Directory(d) => {
                                        if d.name == direction {
                                            Some(d)
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                })
                                .unwrap();
                            dir.fill(iter);
                            self.size += dir.size;
                        }
                    }
                    Some("ls") => {}
                    None | Some(_) => unreachable!(),
                },
                Some("dir") => {
                    let name = split.next().unwrap();
                    self.contents.push(Node::Directory(Directory {
                        name: name.to_owned(),
                        contents: Vec::new(),
                        size: 0,
                    }));
                }
                Some(size) => {
                    let size = size.parse::<u32>().unwrap();

                    self.contents
                        .push(Node::File(split.next().unwrap().to_owned(), size));
                    self.size += size;
                }
            }
        }
    }

    pub fn find_dirs_of_at_most(&self, at_most: u32) -> u32 {
        let res = if self.size > at_most { 0 } else { self.size };
        res + self
            .contents
            .iter()
            .map(|n| match n {
                Node::File(_, _) => 0,
                Node::Directory(d) => d.find_dirs_of_at_most(at_most),
            })
            .sum::<u32>()
    }

    pub fn find_smallest_dir_of_at_least(&self, at_least: u32) -> Option<u32> {
        let i = self.contents.iter().filter_map(|n| match n {
            Node::File(_, _) => None,
            Node::Directory(d) => d.find_smallest_dir_of_at_least(at_least),
        });

        if self.size > at_least {
            i.chain(std::iter::once(self.size)).min()
        } else {
            i.min()
        }
    }
}

impl<S> FromIterator<S> for Directory
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut dir = Self {
            name: "/".to_owned(),
            contents: Vec::new(),
            size: 0,
        };

        dir.fill(&mut iter.into_iter().skip(1));

        dir
    }
}

pub struct Day7 {
    dataset: Dataset,
}

impl Day7 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day7 {
    fn name(&self) -> &'static str {
        "No Space Left On Device"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let directory = data.iter().collect::<Directory>();

        Ok(directory.find_dirs_of_at_most(100000).to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let directory = data.iter().collect::<Directory>();

        let unused_space = TOTAL_SIZE - directory.size;

        Ok(directory
            .find_smallest_dir_of_at_least(REQUIRED_UNUSED_SPACE - unused_space)
            .unwrap()
            .to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
