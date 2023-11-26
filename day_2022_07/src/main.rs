use std::iter::Peekable;

use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
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

/// A directory in the filesystem. It stores the cumulated size of its
/// children directory.
#[derive(Debug)]
struct Dir {
    /// The cumulated size of the files and directories contained
    /// in this directory.
    size: usize,
    /// The list of children directories.
    children: Vec<Dir>,
}

impl Dir {
    /// Create a directory from the problem's input.
    fn new(input: &str) -> Dir {
        // We will recurse for this. Our input is a peekable iterator on the
        // original input's line, as well as the eventual name of the
        // parent directory.
        fn inner<'a, I: Iterator<Item = &'a str>>(
            input: &mut Peekable<I>,
            mut parent_dir_name: Option<&'a str>,
        ) -> Dir {
            // The initial size of the current directory is 0.
            let mut size = 0;
            // It also initially has zero children.
            let mut children = Vec::new();

            // Try to get one line from our input. If there is one...
            while let Some(l) = input.next() {
                // Split the line into whitespace-separated tokens.
                let mut tokens = l.split_ascii_whitespace();
                // The line *has* to be a command, starting with a dollar.
                assert_eq!(tokens.next(), Some("$"));

                // What's the command?
                match tokens.next() {
                    // It's a "cd": let's see what comes next.
                    Some("cd") => match tokens.next() {
                        // A "..": we go up, so we are done with this directory.
                        // Let's break out of the loop.
                        Some("..") => break,
                        // A new directory: we will probably recurse here.
                        Some(dir_name) => {
                            // Does the current directory have a parent?
                            if parent_dir_name.is_some() {
                                // Yes: recurse here and get the resulting directory.
                                let child = inner(input, Some(dir_name));
                                // We add the size of this directory to ours.
                                size += child.size;
                                // Add the directory to our children.
                                children.push(child);
                            } else {
                                // This is the root directory, set the name.
                                parent_dir_name = Some(dir_name)
                            }
                        }
                        // Input should always be well-formed, so we won't have
                        // a "cd" command without any operand.
                        None => unreachable!(),
                    },
                    // It's a "ls": we'll need to parse the command's output.
                    Some("ls") => {
                        // Let's peek into the iterator to see what the next
                        // line is, without actually consuming it.
                        while let Some(i) = input.peek() {
                            // The next line is a command, so we have reached
                            // the end of the "ls" output. Break out of this
                            // inner loop.
                            if i.starts_with('$') {
                                break;
                            }

                            // Actually consume this line from the iterator.
                            let i = input.next().unwrap();

                            // Each line of a "ls" output will be one of these:
                            // - <size_of_file> <file_name>
                            // - dir <name_of_dir>
                            // Let's get that first word and see if it's either
                            // "dir" or the size of a file.
                            let (size_or_dir, _) = i.split_once(' ').unwrap();

                            // If the current line is not a directory listing,
                            // and thus a file...
                            if size_or_dir != "dir" {
                                // Add the size of the file to the cumulated
                                // size of the current directory.
                                size += size_or_dir.parse::<usize>().unwrap();
                            }
                        }
                    }
                    // The input should always have only two commands, "cd" or
                    // "ls", so we can panic here if we get another one.
                    Some(_) => unreachable!(),
                    // If we get nothing, might as well break from the loop.
                    None => break,
                }
            }

            // At the end of the exploration, we can return the new directory.
            Dir { size, children }
        }

        // In order to start the recursion, we provide the iterator on the
        // input, and indicate that we start with a root directory.
        inner(&mut input.lines().peekable(), None)
    }

    /// Compute the sum of the sizes of each directory in this filesystem, only
    /// counting those with a size lower or equal to the given threshold.
    fn sum_with_threshold(&self, max: usize) -> usize {
        // For each of our children...
        self.children
            .iter()
            // ...compute the intermediate value using recursion...
            .map(|d| d.sum_with_threshold(max))
            // ...and sum everything.
            .sum::<usize>()
            // Add the current directory's size if it's less or equal than the 
            // threshold value.
            + if self.size <= max { self.size } else { 0 }
    }

    /// Returns an iterator over all the directories in this filesystem.
    fn dirs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Dir> + 'a> {
        // Iterate over the current directory, as well as...
        Box::new(
            std::iter::once(self).chain(
                // ...for each of the current directory's children...
                self.children
                    .iter()
                    // ...the result of the recursion of this function on this child.
                    .flat_map(|d| d.dirs()),
            ),
        )
    }

    /// Returns the size of the smallest directory that can be deleted to
    /// grant enough space in the filesystem.
    fn sum_of_smallest_deletable_dir(&self) -> usize {
        // The filesystem's total size.
        const FILESYSTEM_SIZE: usize = 70_000_000;
        // The required space for the update.
        const REQUIRED: usize = 30_000_000;

        // The amount of unused space in the filesystem.
        let unused = FILESYSTEM_SIZE - self.size;
        // The amount of space to free in our directory tree.
        let amount_to_free = REQUIRED - unused;

        // For every single directory in our filesystem...
        self.dirs()
            // ...only consider directories that, if deleted, could give
            // enough space for the update.
            .filter_map(|d| {
                if d.size >= amount_to_free {
                    Some(d.size)
                } else {
                    None
                }
            })
            // Sort the results...
            .sorted()
            // ...and get the smallest.
            .next()
            // We're guaranteed to have one in our input.
            .unwrap()
    }
}

fn part_1(input: &str) {
    // For this part, we need to know the sum of the size of all
    // directories with a size lower than 100,000 bytes.
    let dir = Dir::new(input);

    println!("{}", dir.sum_with_threshold(100000));
}

fn part_2(input: &str) {
    // For the second part, we need to deleted the smallest directory that
    // would let us update the system.
    let dir = Dir::new(input);

    let x = dir.sum_of_smallest_deletable_dir();

    println!("{x}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
