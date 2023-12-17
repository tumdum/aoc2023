use anyhow::Result;
use std::{
    ops::Range,
    time::{Duration, Instant},
};

use crate::{dijkstra::dijkstra, input::tokens};

type Pos = crate::pos::Pos<i64>;
type Dir = crate::pos::Pos<i8>;

fn get(input: &[Vec<i64>], p: Pos) -> Option<i64> {
    input
        .get(p.row() as usize)
        .and_then(|r| r.get(p.col() as usize))
        .copied()
}

fn find_best_path(input: &[Vec<i64>], move_range: Range<i8>) -> i64 {
    let neighbours = |(curr_pos, curr_dir): &(Pos, Dir)| -> Vec<((Pos, Dir), i64)> {
        let mut ret = vec![];
        for next_dir in [
            Dir::new(0, 1),
            Dir::new(0, -1),
            Dir::new(1, 0),
            Dir::new(-1, 0),
        ] {
            if &next_dir == curr_dir || next_dir == (*curr_dir * -1) {
                continue;
            }
            for dist in move_range.clone() {
                let next_dist = next_dir * dist;
                let next_pos = *curr_pos + next_dist;
                if let Some(_) = get(input, next_pos) {
                    let mut loss = 0;
                    for i in 0..dist {
                        let next_dist = next_dir * i;
                        let next_pos = *curr_pos + next_dist;
                        loss += get(&input, next_pos).unwrap();
                    }
                    ret.push(((next_pos, next_dir), loss));
                }
            }
        }
        ret
    };

    let start1: (Pos, Dir) = (Pos::new(0, 0), Dir::new(0, 1));
    let start2: (Pos, Dir) = (Pos::new(0, 0), Dir::new(1, 0));
    let paths1 = dijkstra(start1, neighbours);
    let paths2 = dijkstra(start2, neighbours);

    let mut ret = vec![];
    for (start, paths) in [(start1, paths1), (start2, paths2)] {
        for dir in [Dir::new(1, 0), Dir::new(0, 1)] {
            let end = (
                Pos::new(input[0].len() as i64 - 1, input.len() as i64 - 1),
                dir,
            );

            ret.push(
                paths.0.get(&end).unwrap() + get(&input, end.0).unwrap()
                    - get(&input, start.0).unwrap(),
            );
        }
    }

    ret.into_iter().min().unwrap()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<i64>> = tokens(input, None)
        .into_iter()
        .map(|row: String| row.bytes().map(|b| (b - b'0') as i64).collect())
        .collect();

    let s = Instant::now();

    let part1 = find_best_path(&input, 1i8..4);
    let part2 = find_best_path(&input, 4i8..11);

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
