# Advent of Code 🎄
This repository contains my own solutions for the [Advent of Code](https://adventofcode.com), as well as some utilities for creating and working on challenges, and helper functions for input parsing.

## Requirements
### Nix & NixOS
A [flake.nix](./flake.nix) is available. You can run

```sh
nix develop
```

to enter a dev shell with `cargo` and all the necessary libraries.

Alternatively, for systems without flake, a [`shell.nix`](./shell.nix) is available, so you should be able to run this:

```sh
nix-shell
```

There is also an [`.envrc`](./.envrc) available for `direnv` users. Allowing the directory should put you in a development shell.

### Others
You will need to have [Rust](https://www.rust-lang.org/tools/install) installed. No other dependecy needed. This should work on most systems.

## How to use
The first thing to do is to create a file named `.aoc-token` that contains your session token for Advent of Code. This will allow the program to fetch problem inputs from the Advent of Code website. In order to get your session token:
1. Go to a past year problem's input (for example, https://adventofcode.com/2022/day/1/input) while logged in.
2. Open the web inspector (usually with F12 on Firefox, no idea for other browsers. You're doing AoC, you can figure that out yourself).
3. Go to the "network" tab and look for the `GET` request for the `input` file (you may have to reload).
4. In the headers, find the `Cookie` header. It should have a value that looks like `session=...`. Copy the entire string (including "session=") into the `.aoc-token` file.

There are two main crates that are distinct from the solution crates:
- [`aoc-manager`](./aoc-manager/) is a utility binary that allows you to create a new crate for a given day and year, and to watch for changes in a solution crate and compile and run every time a change is detected.
- [`aoc-utils`](./aoc-utils/) is a helper library that contains useful functions to quickly parse AoC input into different formats. This is mainly here to save time when working on the day's problem, and "refined" solutions should just write the parsing in the solution, without relying on this crate.

If the current date is an Advent of Code day, i.e. between December 1st and December 25th of the current year, inclusive, then you can run:

```sh
cargo run -- add
```

To quickly create a crate for the day's problem, add it to the workspace, and begin to watch the sources for changes.

If you want to begin working on a previous day's challenge, you can run:

```sh
# For a day in the current year's Advent of Code
cargo run -- add -d my-day
# For a day in a specific year of Advent of Code
cargo run -- add -d my-day -y my-year
```

All of these commands will do the following:
- Create the crate `day_<year>_<day>`, with a code skeleton specifically made for an AoC problem
- Add this crate to the workspace in [`Cargo.toml`](./Cargo.toml)
- Fetch the problem's input from the Advent of Code website, and cache it in the `.input-cache` directory (so that deleting and re-creating the crate doesn't make too many requests to AoC).
- Begin watching for changes on that crate's `main.rs` file, and trigger a `cargo run` for that crate for every change made.

If you only want to watch an already existing day, you can run the following:

```sh
cargo run -- watch -d my-day -y my-year
```

And if you want to run the standalone solution yourself, you can just specify the binary when using `cargo run`, just like in a normal workspace:

```sh
cargo run --bin day_my-day_my-year
```