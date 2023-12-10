use anyhow::Result;
use rustc_hash::FxHashSet;
use smallvec::{smallvec, SmallVec};
use std::time::{Duration, Instant};

use crate::input::tokens;

type Map = Vec<Vec<char>>;
type PosToAllowed = Vec<SmallVec<[(Pos, SmallVec<[char; 3]>); 4]>>;
type Pos = (i16, i16);

#[inline(always)]
fn get(x: i16, y: i16, m: &Map) -> Option<char> {
    m.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied()
}

#[inline(always)]
fn get_next(
    x: i16,
    y: i16,
    m: &Map,
    seen: &[Vec<bool>],
    p_to_delta: &PosToAllowed,
) -> Option<(i16, i16)> {
    let p = get(x, y, m).unwrap();
    for ((dx, dy), allowed_chars) in &p_to_delta[p as usize] {
        if let Some(current) = get(x + dx, y + dy, m) {
            if allowed_chars.contains(&current) && !seen[(y + dy) as usize][(x + dx) as usize] {
                return Some((x + dx, y + dy));
            }
        }
    }
    None
}

fn flood_fill(
    start: Pos,
    m: &Map,
    todo: &mut Vec<Pos>,
    not_in_loop: &mut Vec<Pos>,
    visited: &mut [bool],
) -> (i64, bool) {
    not_in_loop.clear();
    todo.clear();
    todo.push(start);
    let w: usize = m[0].len();
    let mut reached_outside = false;
    let mut next_id = 0;
    let mut size = 0;
    let mut cands: SmallVec<[Pos; 4]> = Default::default();
    while next_id < todo.len() {
        let (x, y) = todo[next_id];
        let idx = y as usize * w + x as usize;
        if visited[idx] {
            next_id += 1;
            continue;
        }
        not_in_loop.push((x, y));
        size += 1;
        visited[idx] = true;
        cands.clear();

        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let x = x + dx;
            let y = y + dy;

            if x < 0 || y < 0 || x == w as i16 || y == m.len() as i16 {
                reached_outside = true;
            }
            if let Some(current) = get(x, y, m) {
                if current == '.' {
                    if !visited[y as usize * w + x as usize] {
                        todo.push((x, y));
                    }
                }
            }
        }

        next_id += 1;
    }
    (size, reached_outside)
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
                start = (x as i16, y as i16);
            }
            if *val == '.' {
                dots.insert((x as i16, y as i16));
            }
        }
    }

    let mut seen: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    seen[start.1 as usize][start.0 as usize] = true;
    let mut p_to_delta: PosToAllowed = vec![smallvec![]; 256];
    let l = smallvec!['-', 'L', 'F'];
    let r = smallvec!['-', 'J', '7'];
    let u = smallvec!['|', '7', 'F'];
    let d = smallvec!['|', 'L', 'J'];

    p_to_delta['|' as usize] = smallvec![((0, 1), d.clone()), ((0, -1), u.clone())];
    p_to_delta['-' as usize] = smallvec![((1, 0), r.clone()), ((-1, 0), l.clone())];
    p_to_delta['L' as usize] = smallvec![((0, -1), u.clone()), ((1, 0), r.clone())];
    p_to_delta['J' as usize] = smallvec![((0, -1), u.clone()), ((-1, 0), l.clone())];
    p_to_delta['7' as usize] = smallvec![((0, 1), d.clone()), ((-1, 0), l.clone())];
    p_to_delta['F' as usize] = smallvec![((1, 0), r.clone()), ((0, 1), d.clone())];
    p_to_delta['S' as usize] = smallvec![
        ((1, 0), r.clone()),
        ((-1, 0), l.clone()),
        ((0, 1), d.clone()),
        ((0, -1), u.clone()),
    ];

    let mut len = 0;
    let mut current = start;

    loop {
        len += 1;
        match get_next(current.0, current.1, &input, &seen, &p_to_delta) {
            Some(p) => {
                seen[p.1 as usize][p.0 as usize] = true;
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
            if !seen[y as usize][x as usize] {
                input[y][x] = '.';
                dots.insert((x as i16, y as i16));
            }
        }
    }

    let mut part2 = 0;

    let mut seen = vec![false; input.len() * input[0].len()];
    let mut not_in_loop: Vec<Pos> = Vec::with_capacity(seen.len());
    let mut buf = Vec::with_capacity(seen.len());
    while !dots.is_empty() {
        let start = *dots.iter().next().unwrap();

        let (size, reached_outside) =
            flood_fill(start, &input, &mut buf, &mut not_in_loop, &mut seen);

        for p in &not_in_loop {
            dots.remove(p);
        }

        if !reached_outside {
            let start = *not_in_loop.iter().next().unwrap();

            if is_outside(start, &input) {
                part2 += size;
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
