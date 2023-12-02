use anyhow::Result;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::input::tokens;

fn parse(s: &str) -> FxHashMap<String, i64> {
    s.split(", ")
        .map(|g| {
            let mut s = g.split_whitespace();
            let num = s.next().unwrap().parse().unwrap();
            let name = s.next().unwrap().to_owned();
            (name, num)
        })
        .collect()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<FxHashMap<String, i64>>> = tokens(input, Some("\n"))
        .into_iter()
        .flat_map(|l: String| {
            l.split(": ")
                .skip(1)
                .map(|s| s.split("; ").map(parse).collect())
                .next()
        })
        .collect();

    let limits: FxHashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    let s = Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    for (id, line) in input.iter().enumerate() {
        let mut nok = false;
        let mut max: FxHashMap<&str, i64> = Default::default();
        for group in line {
            if group.iter().any(|(name, num)| limits[name.as_str()] < *num) {
                nok = true;
            }
            for (name, num) in group {
                max.entry(name)
                    .and_modify(|m| *m = (*m).max(*num))
                    .or_insert(*num);
            }
        }
        if !nok {
            part1 += id + 1;
        }
        part2 += max.into_values().product::<i64>();
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(2593, part1);
        assert_eq!(54699, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
