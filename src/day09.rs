use anyhow::Result;
use std::time::{Duration, Instant};

use crate::input::token_groups;

fn find_next(nums: &[i64]) -> i64 {
    if nums.iter().all(|v| *v == nums[0]) {
        nums[0]
    } else {
        nums.last().unwrap() + find_next(&nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>())
    }
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<i64>> = token_groups(input, "\n", None);

    let s = Instant::now();

    let part1 = input.iter().map(|nums| find_next(&nums)).sum::<i64>();
    let part2 = input
        .iter()
        .map(|nums| find_next(&nums.into_iter().copied().rev().collect::<Vec<_>>()))
        .sum::<i64>();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(1877825184, part1);
        assert_eq!(1108, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
