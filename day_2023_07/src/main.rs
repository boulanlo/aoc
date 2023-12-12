use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

// Uncomment the line below to include your problem input
// const INPUT: &str = include_str!("input.txt");

// The example given in the prompt.
const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

/// The label given to a hand of 5 cards.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    /// Five of a kind: AAAAA
    FiveKind,
    /// Four of a kind: AJAAA
    FourKind,
    /// Full house, 3 of a kind + 2 of a kind: AJJAA
    FullHouse,
    /// Three of a kind: AJQAA
    ThreeKind,
    /// Two pairs of cards: AAJJQ
    TwoPair,
    /// One pair of cards: AAJQK
    OnePair,
    /// Nothing: AJQK9
    HighCard,
}

// Given a hash map representing the kind of cards of a hand, compute what kind
// of hand it is. For example, the hand 'AAJJQ' would have the corresponding map
// { 'A': 2, 'J': 2, 'Q': 1 }.
fn compute_hand(hand: HashMap<char, usize>) -> Hand {
    // To know what kind of hand we have, we only have to look at the number of
    // every kind of cards.
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
}

// Computes the winnings (the problem's result) from a list of hands and their
// bids.
fn compute_winnings<ComputeHand, CardValue>(
    // The input string
    input: &str,
    // A function that, given a string representing a hand, returns what kind of
    // `Hand` it is.
    compute_hand: ComputeHand,
    // A function that gives the "value" of the card passed as parameter.
    // The lowest the value, the more important the card is.
    card_value: CardValue,
) -> u32
where
    ComputeHand: Fn(&str) -> Hand,
    CardValue: Fn(char) -> usize + Copy,
{
    // For each line in the input...
    input
        .lines()
        .map(|l| {
            // Separate the hand and the bid strings.
            let (hand, bid) = l.split_once(' ').unwrap();

            let hand_map = compute_hand(hand);

            // Return the hand, the map and the parsed bid value.
            (hand, hand_map, bid.parse::<u32>().unwrap())
        })
        // Sort everything in the iterator according to the following rule:
        .sorted_unstable_by(|(a, ha, _), (b, hb, _)| {
            // The most important thing is the kind of hand. The best hand wins.
            match ha.cmp(hb) {
                Ordering::Less => Ordering::Less,
                // However, if both hands are of the same type, we check the
                // cards of the hand from left to right, and pick the one whose
                // card values are the lowest. If at the end everything is equal,
                // both hands are truly equal.
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
        // Reverse the order, so that the best hands are last.
        .rev()
        // Add an enumeration to get the hands' rank.
        .enumerate()
        // Convert each pair of hand, map and bid into the winning, based on the
        // rank.
        .map(|(i, (_, _, bid))| (i as u32 + 1) * bid)
        // And finally, sum everything up.
        .sum::<u32>()
}

fn part_1(input: &str) {
    // In part 1, the hands are computed directly and the card values are the
    // standard Poker ones, with the addition of the 'T' card between 'J' and '9'.
    let result = compute_winnings(
        input,
        // Computing a hand is simply aggregating characters into a hashmap of
        // counts and using `compute_hand()` on it.
        |hand| {
            compute_hand(hand.chars().fold(HashMap::new(), |mut h, c| {
                h.entry(c).and_modify(|v| *v += 1).or_insert(1usize);
                h
            }))
        },
        // The card value is the index in this sorted array of card labels.
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
    // In part 2, the 'J' is now a Joker: as a card in itself, it is now the
    // weakest card, lower than 2; but to compensate, when calculating what
    // kind a hand is, 'J' counts as whatever card would make the hand the best.

    // Card value calculation is mostly the same, except that 'J' is now at the
    // end.
    let card_value = |c| {
        [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ]
        .into_iter()
        .position(|x| c == x)
        .unwrap()
    };

    let result = compute_winnings(
        input,
        // Now, computing a hand is a bit more complex: we need to become the
        // Joker.
        |hand| {
            // Compute the hand as before.
            let mut hand = hand.chars().fold(HashMap::new(), |mut h, c| {
                h.entry(c).and_modify(|v| *v += 1).or_insert(1usize);
                h
            });

            // If there is a Joker in the hand...
            if let Some(v) = hand.remove(&'J') {
                // ...we need to find the best key to use as the joker's
                // replacement.
                let best_key = *hand
                    .iter()
                    // The best key is the one the most present in the hand,
                    // because using the Joker as this card will directly
                    // upgrade it to the next tier. However, if multiple cards
                    // have the same number of occurences, then it's up to the
                    // value of the cards.
                    .sorted_unstable_by(|(ka, va), (kb, vb)| match va.cmp(vb) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Equal => card_value(**ka).cmp(&card_value(**kb)),
                        Ordering::Greater => Ordering::Greater,
                    })
                    // The sort function sorts ascending, so the best one is
                    // the last one.
                    .last()
                    // We can end up in a case where we only have jokers (JJJJJ).
                    // In this case, the only thing we can do is keep the hand
                    // as is.
                    .unwrap_or((&'J', &5))
                    .0;

                // Now, update the best key in the map with the number of jokers
                // we have. If the best key isn't present in the map, it means
                // we're in the JJJJJ case, so just re-insert it back in the map.
                hand.entry(best_key).and_modify(|x| *x += v).or_insert(v);
            }

            // And finally, compute the hand using this joker permutation.
            compute_hand(hand)
        },
        card_value,
    );

    println!("{result}")
}

fn main() {
    let input = EXAMPLE;
    part_1(input);
    part_2(input);
}
