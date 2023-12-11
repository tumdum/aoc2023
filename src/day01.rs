use anyhow::Result;
use std::time::{Duration, Instant};

use crate::input::tokens;

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<String> = tokens(input, None);

    let s = Instant::now();

    let parse = |s: &str| -> u32 {
        let mut i = s.chars().filter(|c| c.is_digit(10));
        let first = i.next().unwrap().to_digit(10).unwrap();
        match i.last().and_then(|c| c.to_digit(10)) {
            Some(last) => first * 10 + last,
            None => first * 10 + first,
        }
    };

    let part1: u32 = lines.iter().map(|s| parse(s)).sum();

    let mut h = vec![];
    h.push(("one", "o1e"));
    h.push(("two", "t2o"));
    h.push(("three", "t3e"));
    h.push(("four", "f4r"));
    h.push(("five", "f5e"));
    h.push(("six", "s6x"));
    h.push(("seven", "s7n"));
    h.push(("eight", "e8t"));
    h.push(("nine", "n9e"));

    let mut input = input.to_owned();
    for (k, v) in &h {
        input = input.replace(k, v);
    }

    let part2: u32 = input.lines().map(|s| parse(&s)).sum();

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
