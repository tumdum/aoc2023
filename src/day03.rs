use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

use crate::input::tokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

fn get(map: &[Vec<char>], pos: Pos) -> Option<char> {
    if let Some(row) = map.get(pos.y as usize) {
        if let Some(val) = row.get(pos.x as usize) {
            return Some(*val);
        }
    }
    None
}

fn get_around(map: &[Vec<char>], pos: Pos) -> (FxHashSet<char>, FxHashSet<Pos>) {
    let d = [-1i64, 0, 1];
    let mut ret: FxHashSet<char> = FxHashSet::default();
    let mut retp: FxHashSet<Pos> = FxHashSet::default();
    for dx in d {
        for dy in d {
            if dx == 0 && dy == 0 {
                continue;
            }
            let p = Pos {
                x: pos.x + dx,
                y: pos.y + dy,
            };
            if let Some(val) = get(map, p) {
                ret.insert(val);
                retp.insert(p);
            }
        }
    }
    (ret, retp)
}

fn collect_num(map: &[Vec<char>], pos: Pos) -> (i64, FxHashSet<Pos>) {
    let mut ret = vec![];
    let mut used = FxHashSet::default();
    used.insert(pos);
    let mut dx = 1i64;
    loop {
        let p = Pos {
            x: pos.x - dx,
            y: pos.y,
        };
        if let Some(c) = get(map, p) {
            if c.is_ascii_digit() {
                ret.push(c);
                used.insert(p);
                dx += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    ret.reverse();
    ret.push(get(map, pos).unwrap());
    dx = 1;
    loop {
        let p = Pos {
            x: pos.x + dx,
            y: pos.y,
        };
        if let Some(c) = get(map, p) {
            if c.is_ascii_digit() {
                ret.push(c);
                used.insert(p);
                dx += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let ret: String = ret.into_iter().collect();
    (ret.parse().unwrap(), used)
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<char>> = tokens::<String>(input, None)
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    let s = Instant::now();

    let mut used: FxHashSet<Pos> = FxHashSet::default();
    let mut adjecent_to_parts: FxHashMap<Pos, Vec<i64>> = Default::default();
    let mut part1 = 0i64;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let p = Pos {
                x: col as i64,
                y: row as i64,
            };
            if used.contains(&p) {
                continue;
            }
            let c = get(&input, p).unwrap();
            if c.is_ascii_digit() {
                let (s, _) = get_around(&input, p);
                let (num, used_for_num) = collect_num(&input, p);
                if s.iter().any(|c| !c.is_ascii_digit() && *c != '.') {
                    part1 += num;
                    used.extend(used_for_num.iter().copied());

                    let mut num_and_around: FxHashSet<Pos> = Default::default();
                    for p in used_for_num {
                        num_and_around.extend(get_around(&input, p).1);
                    }
                    for p in num_and_around {
                        adjecent_to_parts.entry(p).or_default().push(num);
                    }
                }
            }
        }
    }
    let mut part2 = 0i64;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let p = Pos {
                x: col as i64,
                y: row as i64,
            };
            if get(&input, p).unwrap() == '*' {
                if let Some(c) = adjecent_to_parts.get(&p) {
                    if c.len() == 2 {
                        part2 += c[0] * c[1];
                    }
                }
            }
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(540131, part1);
        assert_eq!(86879020, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
