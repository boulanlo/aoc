// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

fn hash<I>(iter: I) -> u32
where
    I: IntoIterator<Item = char>,
{
    iter.into_iter().fold(0u32, |v, c| {
        assert!(c.is_ascii());

        ((v + (c as u32)) * 17) % 256
    })
}

fn part_1(input: &str) {
    let result = input
        .trim()
        .split(',')
        .map(|s| hash(s.chars()))
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part_2(input: &str) {
    let result = input
        .trim()
        .split(',')
        .fold(
            std::array::from_fn(|_| Vec::new()),
            |mut boxes: [Vec<(&str, u8)>; 256], s| {
                if let Some((label, focal_length)) = s.split_once('=') {
                    let hash = hash(label.chars()) as usize;

                    let focal_length = focal_length.parse::<u8>().unwrap();
                    if let Some((_, v)) = boxes[hash].iter_mut().find(|(l, _)| l == &label) {
                        *v = focal_length;
                    } else {
                        boxes[hash].push((label, focal_length));
                    }
                } else {
                    let label = s.split_at(s.chars().count() - 1).0;
                    let hash = hash(label.chars()) as usize;

                    boxes[hash].retain(|(l, _)| l != &label);
                }

                boxes
            },
        )
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(|(j, (_, focus_length))| (i + 1) * (j + 1) * (focus_length as usize))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Part 2: {result}");
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
