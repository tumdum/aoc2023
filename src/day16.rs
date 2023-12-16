use anyhow::Result;
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::{
    mem::swap,
    time::{Duration, Instant},
};

use crate::{input::tokens, vec::StrVec};

type Pos = crate::pos::Pos<i64>;
type Dir = crate::pos::Pos<i8>;

fn get(m: &[StrVec], pos: Pos) -> Option<u8> {
    m.get(pos.y as usize)
        .and_then(|row| row.get(pos.x as usize))
        .copied()
}

fn energized(input: &[StrVec], start: Pos, dir: Dir) -> usize {
    let mut beams: Vec<(Pos, Dir)> = vec![(start, dir)];
    let mut seen_beams: FxHashSet<(Pos, Dir)> = Default::default();

    let mut next_beams: Vec<(Pos, Dir)> = vec![];
    while !beams.is_empty() {
        next_beams.clear();
        for (beam, dir) in &mut beams {
            if !seen_beams.insert((*beam, *dir)) {
                continue;
            }
            let next_pos = *beam + *dir;
            if let Some(next) = get(&input, next_pos) {
                match next {
                    b'.' => {
                        next_beams.push((next_pos, *dir));
                    }
                    b'|' => {
                        if dir.y == 0 {
                            next_beams.push((next_pos, Dir::new(0, 1)));
                            next_beams.push((next_pos, Dir::new(0, -1)));
                        } else {
                            next_beams.push((next_pos, *dir));
                        }
                    }
                    b'-' => {
                        if dir.y == 0 {
                            next_beams.push((next_pos, *dir));
                        } else {
                            next_beams.push((next_pos, Dir::new(1, 0)));
                            next_beams.push((next_pos, Dir::new(-1, 0)));
                        }
                    }
                    b'/' => {
                        next_beams.push((next_pos, Dir::new(-dir.y, -dir.x)));
                    }
                    b'\\' => {
                        next_beams.push((next_pos, Dir::new(dir.y, dir.x)));
                    }
                    c => todo!("c: '{}'", c as char),
                }
            }
        }
        swap(&mut beams, &mut next_beams);
    }

    seen_beams
        .iter()
        .map(|(p, _)| p)
        .collect::<FxHashSet<_>>()
        .len()
        - 1
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<StrVec> = tokens(input, None);

    let s = Instant::now();

    let part1 = energized(&input, Pos::new(-1, 0), Dir::new(1, 0));

    let part2 = (0..input.len())
        .flat_map(|y| {
            [
                (Pos::new(-1, y as i64), Dir::new(1, 0)),
                (Pos::new(input[0].len() as i64, y as i64), Dir::new(-1, 0)),
            ]
        })
        .chain((0..input[0].len()).flat_map(|x| {
            [
                (Pos::new(x as i64, -1), Dir::new(0, 1)),
                (Pos::new(x as i64, input.len() as i64), (Dir::new(0, -1))),
            ]
        }))
        .par_bridge()
        .map(|(start, dir)| energized(&input, start, dir))
        .max()
        .unwrap();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(7951, part1);
        assert_eq!(8148, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
