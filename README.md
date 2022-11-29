# My Advent of Code launcher
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

## The `data` folder
In order to feed data to the challenges, you need to create a `data` folder at the root of this repository (it is not commited
since it may contain the actual answers, and differ from person to person). The architecture of this folder is as follows:

```
data
├─ <year>
│  ├─ <day>
│  │  ├─ example_data.txt
│  │  ├─ example_expected_1.txt
│  │  ├─ example_expected_2.txt
│  │  ├─ real_data.txt
│  │  ├─ real_results_1.txt (optional)
│  │  └─ real_results_2.txt (optional)
│  │ 
│  ├─ <day> 
│  │  ├─ example_data.txt
┆  ┆  ┆
```