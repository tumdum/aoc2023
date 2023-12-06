use anyhow::Result;
use std::time::{Duration, Instant};

use crate::input::token_groups;

fn ways_to_win_race(t: i64, record: i64) -> i64 {
    let first_win = (0..t).position(|p| ((t - p) * p) > record).unwrap() as i64;
    let last_win = t - (0..t).rev().position(|p| ((t - p) * p) > record).unwrap() as i64;

    last_win - first_win
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let races: Vec<Vec<i64>> = token_groups(input, "\n", None);

    let s = Instant::now();

    let part1 = (0..races[0].len())
        .map(|r| ways_to_win_race(races[0][r], races[1][r]))
        .into_iter()
        .product::<i64>();

    let t: i64 = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    let record_distance: i64 = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    let part2 = ways_to_win_race(t, record_distance);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(227850, part1);
        assert_eq!(42948149, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
