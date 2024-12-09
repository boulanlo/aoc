use std::{convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"2333133121414131402"#;

fn checksum<I>(i: I) -> usize
where
    I: IntoIterator<Item = Option<usize>>,
{
    i.into_iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|x| i * x))
        .sum::<usize>()
}

#[derive(Debug, Clone, Copy)]
enum Partition {
    Filled { id: usize, size: usize },
    Empty(usize),
}

struct Disk {
    partitions: Vec<Partition>,
}

impl FromStr for Disk {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Disk {
            partitions: s
                .chars()
                .filter_map(|c| {
                    if c != '\n' {
                        Some(((c as u8) - b'0') as usize)
                    } else {
                        None
                    }
                })
                .enumerate()
                .fold(Vec::new(), |mut v, (i, x)| {
                    if i % 2 == 0 {
                        v.push(Partition::Filled { id: i / 2, size: x })
                    } else {
                        v.push(Partition::Empty(x))
                    }
                    v
                }),
        })
    }
}

impl Disk {
    fn defragment(self) -> usize {
        let mut output = Vec::new();
        let mut buffer = Vec::new();

        let mut iter = self.partitions.iter();

        'outer: while let Some(&p) = iter.next() {
            match p {
                Partition::Filled { id, size } => output.extend(std::iter::repeat(id).take(size)),
                Partition::Empty(mut s) => {
                    while s != 0 {
                        while buffer.is_empty() {
                            match iter.next_back() {
                                None => break 'outer,
                                Some(Partition::Empty(_)) => {}
                                Some(Partition::Filled { id, size }) => {
                                    buffer.extend(std::iter::repeat(*id).take(*size));
                                }
                            }
                        }

                        output.push(buffer.pop().unwrap());
                        s -= 1;
                    }
                }
            }
        }

        output.extend(buffer);

        checksum(output.into_iter().map(Some))
    }

    fn defragment_block(mut self) -> usize {
        let mut left_idx = self.partitions.len() - 1;

        while left_idx != 0 {
            if let Partition::Filled { id: _, size } = self.partitions[left_idx] {
                if let Some(space_idx) =
                    self.partitions
                        .iter()
                        .enumerate()
                        .position(|(space_idx, p)| {
                            if let Partition::Empty(empty_size) = p {
                                space_idx < left_idx && *empty_size >= size
                            } else {
                                false
                            }
                        })
                {
                    let Partition::Empty(empty_size) = self.partitions[space_idx] else {
                        unreachable!()
                    };

                    let remainder = empty_size - size;
                    self.partitions.swap(space_idx, left_idx);

                    if remainder != 0 {
                        self.partitions
                            .insert(space_idx + 1, Partition::Empty(remainder));

                        let Partition::Empty(empty_size) = &mut self.partitions[left_idx + 1]
                        else {
                            unreachable!()
                        };

                        *empty_size -= 1;
                        left_idx += 1;
                    }
                }
            }
            left_idx -= 1;
        }

        checksum(self.partitions.into_iter().flat_map(|p| match p {
            Partition::Filled { id, size } => std::iter::repeat(Some(id)).take(size),
            Partition::Empty(size) => std::iter::repeat(None).take(size),
        }))
    }
}

fn part_1(input: &str) {
    let disk: Disk = input.parse().unwrap();

    let result = disk.defragment();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let disk: Disk = input.parse().unwrap();

    let result = disk.defragment_block();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
