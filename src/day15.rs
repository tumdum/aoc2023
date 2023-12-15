use anyhow::Result;
use smallvec::{smallvec, SmallVec};
use smol_str::SmolStr;
use std::time::{Duration, Instant};

use crate::input::tokens;

fn hash(s: &str) -> i64 {
    let mut ret = 0i64;
    for c in s.chars() {
        ret += (c as u8) as i64;
        ret *= 17;
        ret %= 256;
    }

    ret.try_into().unwrap()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<SmolStr> = tokens(input, Some(","));

    let s = Instant::now();

    let part1: i64 = input.iter().map(|s| hash(&s)).sum();

    let mut boxes: Vec<SmallVec<[(SmolStr, u8); 8]>> = vec![smallvec![]; 256];
    for s in input {
        let label: SmolStr = s.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
        let h = hash(&label);

        if s.ends_with('-') {
            if let Some(idx) = boxes[h as usize].iter().position(|(l, _)| l == &label) {
                boxes[h as usize].remove(idx);
            }
        } else {
            let mut sp = s.split('=');
            sp.next();
            let focal = sp.next().unwrap().parse().unwrap();
            if let Some(idx) = boxes[h as usize].iter().position(|(l, _)| l == &label) {
                boxes[h as usize][idx].1 = focal;
            } else {
                boxes[h as usize].push((label, focal));
            }
        }
    }

    let mut part2 = 0;
    for (box_id, b) in boxes.iter().enumerate() {
        for (f_id, (_label, f)) in b.iter().enumerate() {
            let power = (box_id + 1) * (f_id + 1) * *f as usize;

            part2 += power;
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(503487, part1);
        assert_eq!(261505, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
