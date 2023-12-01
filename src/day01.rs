use anyhow::Result;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::input::tokens;

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<String> = tokens(input, None);

    let s = Instant::now();

    let parse = |s: &str| -> u32 {
        let digs: Vec<_> = s.chars().filter(|c| c.is_digit(10)).collect();
        digs[0].to_digit(10).unwrap() * 10 + digs[digs.len() - 1].to_digit(10).unwrap()
    };

    let part1: u32 = input.iter().map(|s| parse(s)).sum();

    let mut h: FxHashMap<_, _> = FxHashMap::default();
    h.insert("one", "o1e");
    h.insert("two", "t2o");
    h.insert("three", "t3e");
    h.insert("four", "f4r");
    h.insert("five", "f5e");
    h.insert("six", "s6x");
    h.insert("seven", "s7n");
    h.insert("eight", "e8t");
    h.insert("nine", "n9e");

    let part2: u32 = input
        .into_iter()
        .map(|mut s| {
            for (k, v) in &h {
                s = s.replace(k, v);
            }
            parse(&s)
        })
        .sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(54390, part1);
        assert_eq!(54277, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
