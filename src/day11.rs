use anyhow::Result;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

type Pos = (i64, i64);
type Map = Vec<Vec<char>>;

use crate::input::tokens;

fn expand_and_find(m: &Map, mut factor: i64) -> i64 {
    factor -= 1;
    let mut galaxies = vec![];

    for row in 0..m.len() {
        for col in 0..m[row].len() {
            if m[row][col] != '.' {
                galaxies.push((col as i64, row as i64));
            }
        }
    }

    let mut to_expand_by_y = vec![];
    for y in 0..m.len() {
        if m[y].iter().all(|c| *c == '.') {
            to_expand_by_y.push(y as i64);
        }
    }

    let mut expand_y_by_times: FxHashMap<Pos, i64> = Default::default();
    for i in 0..to_expand_by_y.len() {
        for g in &galaxies {
            if g.1 > to_expand_by_y[i] {
                *expand_y_by_times.entry(*g).or_default() += 1;
            }
        }
    }

    for i in 0..galaxies.len() {
        if let Some(times) = expand_y_by_times.get(&galaxies[i]) {
            galaxies[i].1 += times * factor;
        }
    }

    let mut to_expand_x = vec![];
    for x in 0..m[0].len() {
        if m.iter().all(|row| row[x] == '.') {
            to_expand_x.push(x as i64);
        }
    }

    let mut expand_x_by_times: FxHashMap<Pos, i64> = Default::default();
    for i in 0..to_expand_x.len() {
        for g in &galaxies {
            if g.0 > to_expand_x[i] {
                *expand_x_by_times.entry(*g).or_default() += 1;
            }
        }
    }

    for i in 0..galaxies.len() {
        if let Some(mult) = expand_x_by_times.get(&galaxies[i]) {
            galaxies[i].0 += mult * factor;
        }
    }

    let mut ret = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            if i == j {
                continue;
            }

            let dx = (galaxies[i].0 - galaxies[j].0).abs();
            let dy = (galaxies[i].1 - galaxies[j].1).abs();

            ret += dx + dy;
        }
    }

    ret
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<char>> = tokens(input, None)
        .into_iter()
        .map(|s: String| s.chars().collect())
        .collect();

    let s = Instant::now();

    let part1 = expand_and_find(&input, 2);
    let part2 = expand_and_find(&input, 1000000);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(9608724, part1);
        assert_eq!(904633799472, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
