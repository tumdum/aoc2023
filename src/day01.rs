use anyhow::Result;
use std::time::{Duration, Instant};

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    // TODO

    let s = Instant::now();

    // TODO
    // let part1 = todo!();
    // let part2 = todo!();

    let e = s.elapsed();

    if verify_expected {
        // assert_eq!(todo!(), part1);
        // assert_eq!(todo!(), part2);
    }
    if output {
        // println!("\t{}", part1);
        // println!("\t{}", part2);
    }
    Ok(e)
}
