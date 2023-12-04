use anyhow::Result;
use std::{
    mem::swap,
    time::{Duration, Instant},
};

use crate::input::token_groups;

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let div = if input.lines().next().unwrap().find('|').unwrap() == 23 {
        5
    } else {
        10
    };
    let input: Vec<(Vec<i64>, Vec<i64>)> = token_groups(input, "\n", None)
        .into_iter()
        .map(|v| {
            let winning = v[..div].to_vec();
            let have = v[div..].to_vec();
            (winning, have)
        })
        .collect();

    let s = Instant::now();

    let mut cards_won: Vec<usize> = Default::default();
    cards_won.push(0);
    let part1: usize = input
        .iter()
        .map(|(winning, have)| {
            let count = have.into_iter().filter(|c| winning.contains(c)).count();
            cards_won.push(count);
            if count == 0 {
                0
            } else {
                2usize.pow(count as u32 - 1)
            }
        })
        .sum::<usize>();

    let mut todo: Vec<usize> = vec![1; input.len() + 1];
    todo[0] = 0;
    let mut todo_next: Vec<usize> = vec![0; input.len() + 1];

    let mut part2 = 0;
    loop {
        let part2_start = part2;
        for (card, count) in todo.iter().enumerate() {
            part2 += count;
            for next in (1..=cards_won[card]).map(|v| card + v) {
                todo_next[next] += count;
            }
        }
        if part2_start == part2 {
            break;
        }
        swap(&mut todo, &mut todo_next);
        todo_next.fill(0);
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(23678, part1);
        assert_eq!(15455663, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
