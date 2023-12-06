use anyhow::Result;
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

use crate::input::token_groups;

fn binary_search_by<F>(first: i64, last: i64, mut f: F) -> Result<i64, i64>
where
    F: FnMut(i64) -> Ordering,
{
    use Ordering::*;
    let mut size = last - first + 1;
    let mut left = first;
    let mut right = last + 1;
    while left < right {
        let mid = left + size / 2;
        match f(mid) {
            Less => {
                left = mid + 1;
            }
            Greater => {
                right = mid;
            }
            Equal => return Ok(mid),
        }

        size = right - left;
    }

    Err(left)
}

fn either_way<T>(r: Result<T, T>) -> T {
    match r {
        Ok(v) => v,
        Err(v) => v,
    }
}

fn ways_to_win_race(t: i64, record: i64) -> i64 {
    let first_win = either_way(binary_search_by(0, t / 2, |p| ((t - p) * p).cmp(&record)));
    let last_win = either_way(binary_search_by(t / 2, t, |p| record.cmp(&((t - p) * p))));

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
