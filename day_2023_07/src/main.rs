use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

// const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn compute_winnings<KindOfHand, CardValue>(
    input: &str,
    kind_of_hand: KindOfHand,
    card_value: CardValue,
) -> u32
where
    KindOfHand: Fn(&HashMap<char, usize>) -> Hand,
    CardValue: Fn(char) -> usize + Copy,
{
    input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();

            let hand_map = hand.chars().fold(HashMap::new(), |mut h, c| {
                h.entry(c).and_modify(|v| *v += 1).or_insert(1usize);
                h
            });

            (hand, hand_map, bid.parse::<u32>().unwrap())
        })
        .sorted_unstable_by(|(a, ha, _), (b, hb, _)| {
            match kind_of_hand(ha).cmp(&kind_of_hand(hb)) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a
                    .chars()
                    .map(card_value)
                    .zip(b.chars().map(card_value))
                    .find_map(|(a, b)| {
                        let c = a.cmp(&b);
                        if c == Ordering::Equal {
                            None
                        } else {
                            Some(c)
                        }
                    })
                    .unwrap_or(Ordering::Equal),
                Ordering::Greater => Ordering::Greater,
            }
        })
        .rev()
        .enumerate()
        .inspect(|(_, (a, b, c))| println!("{a} {b:?} {c}"))
        .map(|(i, (_, _, bid))| (i as u32 + 1) * bid)
        .sum::<u32>()
}

fn part_1(input: &str) {
    let result = compute_winnings(
        input,
        |hand| match hand.values().collect::<Vec<_>>().as_slice() {
            [5] => Hand::FiveKind,
            [4, 1] | [1, 4] => Hand::FourKind,
            [3, 2] | [2, 3] => Hand::FullHouse,
            [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => Hand::ThreeKind,
            [2, 2, 1] | [2, 1, 2] | [1, 2, 2] => Hand::TwoPair,
            [2, 1, 1, 1] | [1, 2, 1, 1] | [1, 1, 2, 1] | [1, 1, 1, 2] => Hand::OnePair,
            [_, _, _, _, _] => Hand::HighCard,
            _ => unreachable!(),
        },
        |c| {
            [
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ]
            .into_iter()
            .position(|x| c == x)
            .unwrap()
        },
    );

    println!("{result}")
}

fn part_2(input: &str) {
    let card_value = |c| {
        [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ]
        .into_iter()
        .position(|x| c == x)
        .unwrap()
    };

    let kind_of_hand = |hand: &HashMap<char, usize>| {
        let mut hand = hand.clone();
        if let Some(v) = hand.remove(&'J') {
            let best_key = *hand
                .iter()
                .sorted_unstable_by(|(ka, va), (kb, vb)| match va.cmp(vb) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => card_value(**ka).cmp(&card_value(**kb)),
                    Ordering::Greater => Ordering::Greater,
                })
                .last()
                .unwrap_or((&'J', &5))
                .0;

            hand.entry(best_key).and_modify(|x| *x += v).or_insert(v);
        }

        match hand.values().collect::<Vec<_>>().as_slice() {
            [5] => Hand::FiveKind,
            [4, 1] | [1, 4] => Hand::FourKind,
            [3, 2] | [2, 3] => Hand::FullHouse,
            [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => Hand::ThreeKind,
            [2, 2, 1] | [2, 1, 2] | [1, 2, 2] => Hand::TwoPair,
            [2, 1, 1, 1] | [1, 2, 1, 1] | [1, 1, 2, 1] | [1, 1, 1, 2] => Hand::OnePair,
            [_, _, _, _, _] => Hand::HighCard,
            _ => unreachable!(),
        }
    };

    let result = compute_winnings(input, kind_of_hand, card_value);

    println!("{result}")
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
