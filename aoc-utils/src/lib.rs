use std::str::FromStr;

pub fn iter_input_newline_separated(input: &str) -> impl Iterator<Item = &str> {
    input.split("\n\n")
}

pub fn parse_input_newline_separated<T: FromStr>(
    input: &str,
) -> impl Iterator<Item = impl Iterator<Item = T> + '_> + '_
where
    <T as FromStr>::Err: std::fmt::Display + std::fmt::Debug,
{
    input
        .split("\n\n")
        .map(|lines| lines.lines().map(|l| l.parse().unwrap()))
}

pub fn parse_input_unwrapped<T: FromStr>(input: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Display + std::fmt::Debug,
{
    iter_input_unwrapped(input).collect()
}

pub fn iter_input_unwrapped<T: FromStr>(input: &str) -> impl Iterator<Item = T> + '_
where
    <T as FromStr>::Err: std::fmt::Display + std::fmt::Debug,
{
    input.lines().filter_map(|s| {
        if s.trim().is_empty() {
            None
        } else {
            Some(s.parse().unwrap())
        }
    })
}

pub fn parse_input<T: FromStr>(input: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    iter_input(input).collect()
}

pub fn iter_input<T: FromStr>(
    input: &str,
) -> impl Iterator<Item = Result<T, <T as FromStr>::Err>> + '_ {
    input.lines().filter_map(|s| {
        if s.trim().is_empty() {
            None
        } else {
            Some(s.parse())
        }
    })
}

pub fn iter_input_raw(input: &str) -> impl Iterator<Item = &str> {
    inter_input_raw_empty(input).filter(|s| !s.trim().is_empty())
}

pub fn inter_input_raw_empty(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}
