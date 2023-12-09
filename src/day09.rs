use anyhow::Result;
use std::time::{Duration, Instant};

use crate::input::token_groups;

fn find_next(mut nums: Vec<i64>) -> i64 {
    let mut l = nums.len();
    while l > 1 {
        for i in 1..l {
            nums[i - 1] = nums[i] - nums[i - 1];
        }
        l -= 1;
    }
    nums.iter().sum::<i64>()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<i64>> = token_groups(input, "\n", None);

    let s = Instant::now();

    let part1 = input.iter().cloned().map(find_next).sum::<i64>();
    let part2 = input
        .into_iter()
        .map(|mut v| {
            v.reverse();
            find_next(v)
        })
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
