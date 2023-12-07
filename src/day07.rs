use anyhow::Result;
use smallvec::SmallVec;
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

use crate::input::tokens;

type Cards = SmallVec<[u8; 5]>;

const CARDS: [u8; 13] = [
    b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2',
];

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    kind: Kind,
    cards: Cards,
}

impl Hand {
    fn cmp(&self, other: &Self, cards_scores: &[i8; 128]) -> Ordering {
        let r = self.kind.cmp(&other.kind);
        if r != Ordering::Equal {
            return r;
        }

        for i in 0..self.cards.len() {
            let l = cards_scores[self.cards[i] as usize];
            let r = cards_scores[other.cards[i] as usize];
            let ret = l.cmp(&r);
            if ret != Ordering::Equal {
                return ret;
            }
        }

        unreachable!()
    }
}

fn parse(s: &str) -> Hand {
    assert!(s.len() == 5);
    let cards: Cards = s.bytes().collect();

    let k = kind(&cards);
    Hand { kind: k, cards }
}

fn kind(cards: &Cards) -> Kind {
    let mut counts = [0u8; 128];
    for c in cards {
        counts[*c as usize] += 1;
    }
    let mut counts: SmallVec<[u8; 5]> = CARDS
        .iter()
        .filter_map(|c| {
            if counts[*c as usize] > 0 {
                Some(counts[*c as usize])
            } else {
                None
            }
        })
        .collect();
    if counts.len() == 1 {
        return Kind::Five;
    }
    if counts.len() == 5 {
        return Kind::HighCard;
    }
    counts.sort_unstable();

    if counts.last().unwrap() == &4 {
        Kind::Four
    } else if counts[0] == 2 && counts[1] == 3 {
        Kind::Full
    } else if counts.last().unwrap() == &3 {
        Kind::Three
    } else if counts.len() > 2 && counts[1] == 2 && counts[2] == 2 {
        Kind::TwoPair
    } else {
        Kind::OnePair
    }
}

fn find_best_kind(h: &Hand) -> Kind {
    if h.cards.iter().any(|c| *c == b'J') {
        CARDS
            .iter()
            .map(|c| {
                let tmp: Cards = h
                    .cards
                    .iter()
                    .map(|cc| if *cc == b'J' { *c } else { *cc })
                    .collect();
                kind(&tmp)
            })
            .max()
            .unwrap()
    } else {
        h.kind
    }
}

fn total_winnings(hands: impl IntoIterator<Item = (Hand, i64)>) -> i64 {
    hands
        .into_iter()
        .enumerate()
        .map(|(id, (_, bid))| (id as i64 + 1) * bid)
        .sum()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let mut scores = [-100i8; 128];
    scores[b'2' as usize] = 0;
    scores[b'3' as usize] = 1;
    scores[b'4' as usize] = 2;
    scores[b'5' as usize] = 3;
    scores[b'6' as usize] = 4;
    scores[b'7' as usize] = 5;
    scores[b'8' as usize] = 6;
    scores[b'9' as usize] = 7;
    scores[b'T' as usize] = 8;
    scores[b'J' as usize] = 9;
    scores[b'Q' as usize] = 10;
    scores[b'K' as usize] = 11;
    scores[b'A' as usize] = 12;

    let input: Vec<String> = tokens(input, None);
    let input: Vec<(Hand, i64)> = input
        .chunks(2)
        .map(|v| {
            let hand = parse(&v[0]);
            let bid = v[1].parse().unwrap();
            (hand, bid)
        })
        .collect();
    let s = Instant::now();

    let mut hands = input.clone();
    hands.sort_unstable_by(|(left_hand, _), (right_hand, _)| left_hand.cmp(&right_hand, &scores));
    let part1 = total_winnings(hands);

    let mut hands: Vec<(Hand, i64)> = input
        .clone()
        .into_iter()
        .map(|(mut h, bid)| {
            h.kind = find_best_kind(&h);
            (h, bid)
        })
        .collect();

    scores[b'J' as usize] = -1;
    hands.sort_unstable_by(|(left_hand, _), (right_hand, _)| left_hand.cmp(&right_hand, &scores));
    let part2 = total_winnings(hands);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(241344943, part1);
        assert_eq!(243101568, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
