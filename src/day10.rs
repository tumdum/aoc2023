use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

use crate::input::tokens;

type Map = Vec<Vec<char>>;
type PosToAllowed = FxHashMap<char, Vec<Pos>>;
type Pos = (i64, i64);

fn get(x: i64, y: i64, m: &Map) -> Option<char> {
    m.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied()
}

fn get_around(x: i64, y: i64, m: &Map, p_to_delta: &PosToAllowed) -> Vec<Pos> {
    let mut ret = vec![];
    let p = get(x, y, m).unwrap();
    let allowed = p_to_delta.get(&p).unwrap();
    for (dx, dy) in allowed {
        if let Some(current) = get(x + dx, y + dy, m) {
            let mut ok = false;
            if *dx == -1 && (current == '-' || current == 'L' || current == 'F') {
                ok = true;
            }
            if *dx == 1 && (current == '-' || current == 'J' || current == '7') {
                ok = true;
            }
            if *dy == -1 && (current == '|' || current == 'F' || current == '7') {
                ok = true;
            }
            if *dy == 1 && (current == '|' || current == 'J' || current == 'L') {
                ok = true;
            }
            if ok {
                ret.push((x + dx, y + dy));
            }
        }
    }
    ret
}
fn get_around2(x: i64, y: i64, m: &Map, p_to_delta: &PosToAllowed) -> (Vec<Pos>, bool) {
    let mut ret = vec![];
    let mut reached_outside = false;
    let p = get(x, y, m).unwrap();
    let allowed = p_to_delta.get(&p).unwrap();
    for (dx, dy) in allowed {
        let x = x + dx;
        let y = y + dy;

        if x < 0 || y < 0 || x == m[0].len() as i64 || y == m.len() as i64 {
            reached_outside = true;
        }
        if let Some(current) = get(x, y, m) {
            if current == '.' {
                ret.push((x, y));
            }
        }
    }

    (ret, reached_outside)
}

fn get_next(
    x: i64,
    y: i64,
    m: &Map,
    seen: &FxHashSet<Pos>,
    p_to_delta: &PosToAllowed,
) -> Option<(i64, i64)> {
    for p in get_around(x, y, m, p_to_delta) {
        if !seen.contains(&p) {
            return Some(p);
        }
    }
    None
}

fn flood_fill(start: Pos, m: &Map, p_to_delta: &PosToAllowed) -> (FxHashSet<Pos>, bool) {
    let mut not_in_loop: FxHashSet<Pos> = Default::default();
    let mut todo: FxHashSet<Pos> = Default::default();
    todo.insert(start);
    let mut reached_outside = false;
    while !todo.is_empty() {
        let next: Pos = *todo.iter().next().unwrap();
        todo.remove(&next);
        not_in_loop.insert(next);
        let (cands, outside) = get_around2(next.0, next.1, &m, &p_to_delta);
        if outside {
            reached_outside = true;
        }

        for cand in cands {
            if !not_in_loop.contains(&cand) {
                todo.insert(cand);
            }
        }
    }
    (not_in_loop, reached_outside)
}

fn is_outside(start: Pos, m: &Map) -> bool {
    let mut curr_y = start.1;
    let mut seen_tiles = vec![];
    while curr_y >= 0 {
        let c = m[curr_y as usize][start.0 as usize];

        seen_tiles.push(c);

        curr_y -= 1;
    }
    let mut outside = false;

    let mut first_b = '.';
    for c in seen_tiles.iter() {
        match *c {
            '-' => {
                outside = !outside;
            }
            'S' => {
                outside = !outside;
            }
            '|' => {}
            'F' => {
                if first_b == 'J' {
                    outside = !outside;
                    first_b = '.';
                }
            }
            '7' => {
                if first_b == 'L' {
                    outside = !outside;
                    first_b = '.';
                }
            }
            'L' => {
                first_b = 'L';
            }
            'J' => {
                first_b = 'J';
            }
            '.' => {}

            _ => panic!("{}", c),
        }
    }
    outside
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let mut input: Vec<Vec<char>> = tokens(input, None)
        .into_iter()
        .map(|s: String| s.chars().collect())
        .collect();

    let s = Instant::now();

    let mut start = (0, 0);
    let mut dots = FxHashSet::default();
    for (y, row) in input.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == 'S' {
                start = (x as i64, y as i64);
                // break 'l1;
            }
            if *val == '.' {
                dots.insert((x as i64, y as i64));
            }
        }
    }

    let mut seen: FxHashSet<(i64, i64)> = FxHashSet::default();
    seen.insert(start);
    let mut p_to_delta: PosToAllowed = Default::default();
    p_to_delta.insert('|', vec![(0, 1), (0, -1)]);
    p_to_delta.insert('-', vec![(1, 0), (-1, 0)]);
    p_to_delta.insert('L', vec![(0, -1), (1, 0)]);
    p_to_delta.insert('J', vec![(0, -1), (-1, 0)]);
    p_to_delta.insert('7', vec![(0, 1), (-1, 0)]);
    p_to_delta.insert('F', vec![(1, 0), (0, 1)]);
    p_to_delta.insert('S', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);
    p_to_delta.insert('.', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);

    let mut len = 0;
    let mut current = start;

    loop {
        len += 1;
        match get_next(current.0, current.1, &input, &seen, &p_to_delta) {
            Some(p) => {
                seen.insert(p);
                current = p;
            }
            None => {
                break;
            }
        }
    }
    let part1 = len / 2;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if !seen.contains(&(x as i64, y as i64)) {
                input[y][x] = '.';
                dots.insert((x as i64, y as i64));
            }
        }
    }

    let mut part2 = 0;

    while !dots.is_empty() {
        let start = *dots.iter().next().unwrap();

        let (not_in_loop, reached_outside) = flood_fill(start, &input, &p_to_delta);

        for p in &not_in_loop {
            dots.remove(p);
        }

        if !reached_outside {
            let start = *not_in_loop.iter().next().unwrap();

            if is_outside(start, &input) {
                part2 += not_in_loop.len();
            }
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(7012, part1);
        assert_eq!(395, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
