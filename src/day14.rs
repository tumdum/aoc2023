use anyhow::Result;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::input::tokens;

type Pos = (i64, i64);

fn move_by(m: &mut [Vec<u8>], start: Pos, dir: Pos) {
    let mut curr = start;
    assert!(m[curr.1 as usize][curr.0 as usize] == b'O');
    loop {
        let next = (curr.0 + dir.0, curr.1 + dir.1);

        if next.0 < 0 || next.1 < 0 || next.0 >= m[0].len() as i64 || next.1 >= m.len() as i64 {
            m[start.1 as usize][start.0 as usize] = b'.';
            m[curr.1 as usize][curr.0 as usize] = b'O';
            return;
        }
        let at_next = m[next.1 as usize][next.0 as usize];

        if at_next == b'#' || at_next == b'O' {
            m[start.1 as usize][start.0 as usize] = b'.';
            m[curr.1 as usize][curr.0 as usize] = b'O';
            return;
        }
        curr = next;
    }
}

fn move_rocks(m: &mut [Vec<u8>], dir: Pos) {
    for row in 0..m.len() {
        for col in 0..m[row].len() {
            if m[row][col] == b'O' {
                move_by(m, (col as i64, row as i64), dir);
            }
        }
    }
}
fn move_rocks2(m: &mut [Vec<u8>], dir: Pos) {
    for row in (0..m.len()).rev() {
        for col in (0..m[row].len()).rev() {
            if m[row][col] == b'O' {
                move_by(m, (col as i64, row as i64), dir);
            }
        }
    }
}

fn total_load(m: &[Vec<u8>]) -> i64 {
    let mut ret = 0;
    for row in 0..m.len() {
        for col in 0..m[row].len() {
            if m[row][col] == b'O' {
                ret += m.len() - row;
            }
        }
    }
    ret as i64
}

fn cycle_map(map: &mut [Vec<u8>]) {
    move_rocks(map, (0, -1));
    move_rocks(map, (-1, 0));
    move_rocks2(map, (0, 1));
    move_rocks2(map, (1, 0));
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<u8>> = tokens(input, None)
        .into_iter()
        .map(|row: String| row.bytes().collect())
        .collect();

    let s = Instant::now();

    let mut map = input.clone();
    move_rocks(&mut map, (0, -1));
    let part1 = total_load(&map);

    let mut seen: FxHashMap<Vec<Vec<u8>>, usize> = Default::default();

    let mut map = input.clone();
    let total = 1000000000;
    for cycle in 0..total {
        if let Some(c) = seen.get(&map) {
            let left = total - cycle;
            let times = left / (cycle - c);
            let todo = left - (times * (cycle - c));
            for _ in 0..todo {
                cycle_map(&mut map);
            }

            break;
        }
        seen.insert(map.to_vec(), cycle);
        cycle_map(&mut map);
    }
    let part2 = total_load(&map);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(110274, part1);
        assert_eq!(90982, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
