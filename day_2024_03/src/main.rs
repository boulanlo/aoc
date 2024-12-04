use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, one_of},
    combinator::{map, recognize},
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

enum Instruction {
    Mul(u32),
    Cond(bool),
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("mul"),
            delimited(
                tag("("),
                separated_pair(decimal, tag(","), decimal),
                tag(")"),
            ),
        ),
        |(a, b)| Instruction::Mul(a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()),
    )(input)
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    (map(tag("do()"), |_| Instruction::Cond(true)))(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    (map(tag("don't()"), |_| Instruction::Cond(false)))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    (parse_mul)(input)
}

fn parse_instruction_conditional(input: &str) -> IResult<&str, Instruction> {
    (alt((parse_mul, parse_do, parse_dont)))(input)
}

fn parse_program<F>(input: &str, instr_fn: F) -> u32
where
    F: Fn(&str) -> IResult<&str, Instruction>,
{
    (map(many1(map(many_till(anychar, instr_fn), |(_, x)| x)), |v| {
        v.into_iter()
            .fold((0, true), |(s, c), x| match x {
                Instruction::Mul(x) => (s + c.then_some(x).unwrap_or_default(), c),
                Instruction::Cond(b) => (s, b),
            })
            .0
    }))(input)
    .unwrap()
    .1
}

fn part_1(input: &str) {
    let result = parse_program(input, parse_instruction);

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let result = parse_program(input, parse_instruction_conditional);

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
