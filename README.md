# My Advent of Code launcher
> :warning: **This repository contains the solutions of some, or all days of the Advent of Code**. Proceed at your own risk.

This is a little project thrown together for the 2022 edition of the [Advent of Code](https://adventofcode.com).
It features a Terminal User Interface (TUI) that displays, for a given year, all the available challenges that
were implemented, and allows you to run one, or all of them at once. You can run each challenge normally, or assert
that it passes the example given in the prompt (or even the actual result if you already solved it and are trying to
create a better solution).

It's really rough and a lot of things that ought to be command line options are hardcoded. I'll hopefully update everything
as I go.

## Requirements
- Rust (any semi-recent version will do)

## How to run it
`cargo run`

## Commands
- `q` or `<Esc>`: Quit
- `<Left>` and `<Right>` keys: Navigate between the challenge list and the dataset
- `<Up>` and `<Down>`: Scroll lists and text when applicable
- `l`: Go to the list
- `d`: Go to the dataset
- `o`: Go to the outputs
- `a`: Run all days available
- `<Space>` when on a challenge: toggle between expanding into parts, or collapsing into a single day
- `r`: Run a single day or part (coming soon!)

## The `data` folder
In order to feed data to the challenges, you need to create a `data` folder at the root of this repository (it is not commited
since it may contain the actual answers, and differ from person to person). The architecture of this folder is as follows:

```
data
├─ <year>
│  ├─ <day>
│  │  ├─ example_data.txt
│  │  ├─ example_expected_1.txt
│  │  ├─ example_expected_2.txt (optional)
│  │  ├─ real_data.txt
│  │  ├─ real_results_1.txt (optional)
│  │  └─ real_results_2.txt (optional)
│  │ 
│  ├─ <day> 
│  │  ├─ example_data.txt
┆  ┆  ┆
```