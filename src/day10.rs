use anyhow::Result;
use rustc_hash::FxHashSet;
use smallvec::{smallvec, SmallVec};
use std::time::{Duration, Instant};

use crate::input::tokens;

type Map = Vec<Vec<char>>;
type PosToAllowed = Vec<SmallVec<[(Pos, SmallVec<[char; 3]>); 4]>>;
type Pos = (i16, i16);

fn get(x: i16, y: i16, m: &Map) -> Option<char> {
    m.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied()
}

fn get_around(x: i16, y: i16, m: &Map, p_to_delta: &PosToAllowed) -> SmallVec<[Pos; 4]> {
    let mut ret = smallvec![];
    let p = get(x, y, m).unwrap();
    let allowed = &p_to_delta[p as usize];
    for ((dx, dy), allowed_chars) in allowed {
        if let Some(current) = get(x + dx, y + dy, m) {
            if allowed_chars.contains(&current) {
                ret.push((x + dx, y + dy));
            }
        }
    }
    ret
}
fn get_around_dot(x: i16, y: i16, m: &Map) -> (SmallVec<[Pos; 4]>, bool) {
    let mut ret = smallvec![];
    let mut reached_outside = false;

    for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        let x = x + dx;
        let y = y + dy;

        if x < 0 || y < 0 || x == m[0].len() as i16 || y == m.len() as i16 {
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
    x: i16,
    y: i16,
    m: &Map,
    seen: &[Vec<bool>],
    p_to_delta: &PosToAllowed,
) -> Option<(i16, i16)> {
    for p in get_around(x, y, m, p_to_delta) {
        if !seen[p.1 as usize][p.0 as usize] {
            return Some(p);
        }
    }
    None
}

fn flood_fill(start: Pos, m: &Map) -> (FxHashSet<Pos>, bool) {
    let mut not_in_loop: FxHashSet<Pos> = Default::default();
    let mut todo: FxHashSet<Pos> = Default::default();
    todo.insert(start);
    let mut reached_outside = false;
    while !todo.is_empty() {
        let next: Pos = *todo.iter().next().unwrap();
        todo.remove(&next);
        not_in_loop.insert(next);
        let (cands, outside) = get_around_dot(next.0, next.1, &m);
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

    while !dots.is_empty() {
        let start = *dots.iter().next().unwrap();

        let (not_in_loop, reached_outside) = flood_fill(start, &input);

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
