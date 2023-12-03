use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
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

fn get_around(map: &[Vec<char>], pos: Pos) -> FxHashMap<Pos, char> {
    let d = [-1i64, 0, 1];
    let mut ret: FxHashMap<Pos, char> = Default::default();
    ret.reserve(8);
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
                ret.insert(p, val);
            }
        }
    }
    ret
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<char>> = tokens::<String>(input, None)
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    let s = Instant::now();

    let mut part1 = 0i64;
    let mut current_digs = vec![];
    let mut current_pos = vec![];
    let mut gears: FxHashMap<Pos, FxHashSet<i64>> = Default::default();
    for (row_id, row) in input.iter().enumerate() {
        for (col_id, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                current_digs.push(*c);
                let p = Pos {
                    x: col_id as i64,
                    y: row_id as i64,
                };
                current_pos.push(p);
            }
            if (!c.is_ascii_digit() || col_id == (row.len() - 1)) && !current_digs.is_empty() {
                let mut is_part_num = false;
                let mut candidate_gears: SmallVec<[Pos; 12]> = Default::default();
                for p in current_pos.iter() {
                    for (p, c) in get_around(&input, *p) {
                        if !c.is_ascii_digit() && c != '.' {
                            is_part_num = true;
                        }
                        if c == '*' {
                            candidate_gears.push(p);
                        }
                    }
                }
                if is_part_num {
                    let num: i64 = current_digs
                        .iter()
                        .copied()
                        .fold(0, |a, v| a * 10 + v.to_digit(10).unwrap() as i64);
                    for c in candidate_gears {
                        gears.entry(c).or_default().insert(num);
                    }
                    part1 += num;
                }
                current_digs.clear();
                current_pos.clear();
            }
        }
    }
    let part2 = gears
        .into_values()
        .filter(|v| v.len() == 2)
        .map(|v| v.into_iter().product::<i64>())
        .sum::<i64>();
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
