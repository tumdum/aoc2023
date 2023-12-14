use anyhow::Result;
use itertools::iproduct;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::input::tokens;

type Pos = (i64, i64);
type Map = Vec<Vec<u8>>;

fn move_by(m: &mut Map, start: Pos, dir: Pos) {
    let mut curr = start;
    debug_assert!(m[curr.1 as usize][curr.0 as usize] == b'O');
    loop {
        let next = (curr.0 + dir.0, curr.1 + dir.1);

        if next.0 < 0 || next.1 < 0 || next.0 >= m[0].len() as i64 || next.1 >= m.len() as i64 {
            break;
        }
        let at_next = m[next.1 as usize][next.0 as usize];

        if at_next == b'#' || at_next == b'O' {
            break;
        }
        curr = next;
    }
    m[start.1 as usize][start.0 as usize] = b'.';
    m[curr.1 as usize][curr.0 as usize] = b'O';
}

fn move_rocks(
    m: &mut Map,
    dir: Pos,
    rows: impl Iterator<Item = usize>,
    cols: impl Iterator<Item = usize> + Clone,
) {
    for (row, col) in iproduct!(rows, cols) {
        if m[row][col] == b'O' {
            move_by(m, (col as i64, row as i64), dir);
        }
    }
}

fn total_load(m: &Map) -> usize {
    iproduct!(0..m.len(), 0..m[0].len())
        .filter(|(row, col)| m[*row][*col] == b'O')
        .map(|(row, _)| m.len() - row)
        .sum()
}

fn cycle_map(map: &mut Map) {
    let l = map.len();
    let ll = map[0].len();
    move_rocks(map, (0, -1), 0..l, 0..ll);
    move_rocks(map, (-1, 0), 0..l, 0..ll);
    move_rocks(map, (0, 1), (0..l).rev(), (0..ll).rev());
    move_rocks(map, (1, 0), (0..l).rev(), (0..ll).rev());
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Map = tokens(input, None)
        .into_iter()
        .map(|row: String| row.bytes().collect())
        .collect();

    let s = Instant::now();

    let mut map = input.clone();
    let l = map.len();
    let ll = map[0].len();
    move_rocks(&mut map, (0, -1), 0..l, 0..ll);
    let part1 = total_load(&map);

    let mut seen: FxHashMap<Vec<Vec<u8>>, usize> = Default::default();

    let mut map = input.clone();
    let total = 1000000000;
    for cycle in 0..total {
        if let Some(previous_cycle) = seen.get(&map) {
            let left = total - cycle;
            let cycle_len = cycle - previous_cycle;
            let times = left / cycle_len;
            let todo = left - (times * cycle_len);
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
