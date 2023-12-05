use anyhow::Result;
use std::{
    ops::Range,
    time::{Duration, Instant},
};

use crate::input::{token_groups, tokens};

fn convert(maps: &[Vec<(Range<i64>, i64)>], mut num: i64) -> i64 {
    for map in maps {
        for (range, dst) in map {
            if range.contains(&num) {
                num = dst + num - range.start;
                break;
            }
        }
    }
    num
}

fn convert_inv(maps: &[Vec<(Range<i64>, i64)>], num: i64) -> Vec<i64> {
    (0..maps.len()).map(|d| convert(&maps[d..], num)).collect()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let maps: Vec<Vec<(i64, i64, i64)>> = token_groups(input, "\n\n", None)
        .into_iter()
        .skip(1)
        .map(|map| {
            let map: Vec<i64> = map
                .into_iter()
                .skip(2)
                .map(|v: String| v.parse().unwrap())
                .collect();
            map.chunks(3).map(|v| (v[0], v[1], v[2])).collect()
        })
        .collect();

    let seeds: Vec<i64> = tokens(input.lines().next().unwrap(), None);

    let s = Instant::now();

    let maps: Vec<Vec<(Range<i64>, i64)>> = maps
        .iter()
        .map(|v| {
            v.iter()
                .map(|(dst, src, len)| ((*src..(src + len), *dst)))
                .collect()
        })
        .collect();

    let part1 = seeds.iter().map(|s| convert(&maps, *s)).min().unwrap();

    let mut maps_inv: Vec<Vec<(Range<i64>, i64)>> = maps.to_vec();
    maps_inv.reverse();
    let maps_inv: Vec<Vec<(Range<i64>, i64)>> = maps_inv
        .iter()
        .map(|map| {
            map.iter()
                .map(|(r, dst)| {
                    let len = r.end - r.start as i64;
                    let src = r.start;
                    (*dst..(dst + len), src)
                })
                .collect()
        })
        .collect();

    let very_important_numbers: Vec<i64> = maps
        .iter()
        .flat_map(|map| map.iter().map(|(r, _)| r.clone()))
        .flat_map(|r| [r.start, r.clone().last().unwrap()])
        .flat_map(|v| convert_inv(&maps_inv, v))
        .collect();

    let part2 = seeds
        .chunks(2)
        .map(|c| {
            let r = c[0]..(c[0] + c[1]);
            let mut to_check = very_important_numbers.clone();
            to_check.push(c[0]);
            to_check.push(c[0] + c[1] - 1);
            to_check
                .into_iter()
                .flat_map(|p| {
                    if r.contains(&p) {
                        Some(convert(&maps, p))
                    } else {
                        None
                    }
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(621354867, part1);
        assert_eq!(15880236, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
