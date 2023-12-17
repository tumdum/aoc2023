use anyhow::Result;
use rayon::prelude::*;
use smallvec::{smallvec, SmallVec};
use std::{
    ops::Range,
    time::{Duration, Instant},
};

use crate::{dijkstra::dijkstra, input::tokens};

type Pos = crate::pos::Pos<i16>;
type Dir = crate::pos::Pos<i8>;

const ALL_DIRS: [Dir; 4] = [
    Dir::new(0, 1),
    Dir::new(0, -1),
    Dir::new(1, 0),
    Dir::new(-1, 0),
];

fn get(input: &[Vec<i64>], p: Pos) -> Option<i64> {
    input
        .get(p.row() as usize)
        .and_then(|r| r.get(p.col() as usize))
        .copied()
}

fn find_best_path(input: &[Vec<i64>], move_range: Range<i8>) -> i64 {
    let neighbours = |(curr_pos, curr_dir): &(Pos, Dir)| -> SmallVec<[((Pos, Dir), i64); 14]> {
        let mut ret: SmallVec<[_; 14]> = smallvec![];
        for next_dir in ALL_DIRS {
            if &next_dir == curr_dir || next_dir == (*curr_dir * -1) {
                continue;
            }
            for dist in move_range.clone() {
                let next_dist = next_dir * dist;
                let next_pos = *curr_pos + next_dist;
                if get(input, next_pos).is_some() {
                    let loss: i64 = (1..=dist)
                        .map(|d| get(&input, *curr_pos + (next_dir * d)).unwrap())
                        .sum();
                    ret.push(((next_pos, next_dir), loss));
                }
            }
        }
        ret
    };

    let paths = dijkstra(
        &[
            (Pos::new(0, 0), Dir::new(0, 1)),
            (Pos::new(0, 0), Dir::new(1, 0)),
        ],
        neighbours,
    );

    let mut ret = vec![];

    for dir in [Dir::new(1, 0), Dir::new(0, 1)] {
        let end = (
            Pos::new(input[0].len() as i16 - 1, input.len() as i16 - 1),
            dir,
        );

        ret.push(*paths.0.get(&end).unwrap());
    }

    ret.into_iter().min().unwrap()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<i64>> = tokens(input, None)
        .into_iter()
        .map(|row: String| row.bytes().map(|b| (b - b'0') as i64).collect())
        .collect();

    let s = Instant::now();

    let parts: Vec<_> = [(1i8..4), (4i8..11)]
        .par_iter()
        .map(|moves| find_best_path(&input, moves.clone()))
        .collect();
    let part1 = parts[0];
    let part2 = parts[1];

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(902, part1);
        assert_eq!(1073, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
