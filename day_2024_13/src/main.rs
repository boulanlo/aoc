use std::{convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("input.txt");
const _EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

fn parse_number_couple(s: &str) -> (usize, usize) {
    let (a, b, _) = s
        .chars()
        .filter(|c| *c == ',' || (*c as u8).is_ascii_digit())
        .fold((0, 0, false), |(mut a, mut b, mut right), c| {
            if c == ',' {
                right = true;
            } else {
                let c = (c as u8 - b'0') as usize;
                let x = if right { &mut b } else { &mut a };

                *x *= 10;
                *x += c;
            }

            (a, b, right)
        });

    (a, b)
}

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl FromStr for Machine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut couples = s.lines().map(parse_number_couple);
        let a = couples.next().unwrap();
        let b = couples.next().unwrap();
        let prize = couples.next().unwrap();

        Ok(Self { a, b, prize })
    }
}

impl Machine {
    pub const fn solve(&self, bias: usize) -> Option<usize> {
        // x1*a + x2*b = px
        // y1*a + y2*b = py

        let (mut x1, mut y1) = (self.a.0 as isize, self.a.1 as isize);
        let (mut x2, y2) = (self.b.0 as isize, self.b.1 as isize);
        let (mut px, mut py) = (
            (self.prize.0 + bias) as isize,
            (self.prize.1 + bias) as isize,
        );

        let (n_x2, n_y2) = (-x2, y2);

        x1 *= n_y2;
        x2 *= n_y2;
        px *= n_y2;

        y1 *= n_x2;
        py *= n_x2;

        let x3 = x1 + y1;
        let p3 = px + py;

        if p3 % x3 != 0 {
            return None;
        }

        let a = (p3 / x3) as usize;

        px -= x1 * a as isize;

        if px % x2 != 0 {
            return None;
        }

        let b = (px / x2) as usize;

        Some(3 * a + b)
    }
}

fn part_1(input: &str) {
    let result = input
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .filter_map(|m: Machine| m.solve(0))
        .sum::<usize>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let result = input
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .filter_map(|m: Machine| m.solve(10000000000000))
        .sum::<usize>();

    println!("Part 2: {result}");
}

fn main() {
    let input = INPUT;
    part_1(input);
    part_2(input);
}
