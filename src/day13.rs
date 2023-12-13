use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;
use smallvec::{SmallVec, ToSmallVec};
use std::time::{Duration, Instant};

use crate::{input::token_groups, vec::transpose};

type Row = SmallVec<[u8; 20]>;

fn find_mirror(m: &[Row], not_this: Option<usize>) -> Option<usize> {
    let mut longest = None;
    for col in 1..m[0].len() {
        let mut mirrored_rows = 0;
        for row in 0..m.len() {
            let mut left: Row = m[row][..col].to_smallvec();
            left.reverse();
            assert!(!left.is_empty());

            let right = &m[row][col..];
            assert!(!right.is_empty());

            let mut any = false;
            if left.len() >= right.len() {
                if left.starts_with(&right) {
                    mirrored_rows += 1;
                    any = true;
                }
            } else {
                if right.starts_with(&left) {
                    mirrored_rows += 1;
                    any = true;
                }
            }
            if !any {
                break;
            }
        }
        if mirrored_rows == m.len() {
            if not_this != Some(col) {
                longest = Some(col);
            }
        }
    }

    longest
}

fn flip(m: &mut [Row], row: usize, col: usize) {
    let v = m[row][col];
    let new = if v == b'.' { b'#' } else { b'.' };
    m[row][col] = new;
}

fn find_smudge(
    map: &[Row],
    old_by_col: Option<usize>,
    old_by_row: Option<usize>,
) -> (Option<usize>, Option<usize>) {
    let mut copy = map.to_vec();
    for (row, col) in iproduct!(0..map.len(), 0..map[0].len()) {
        flip(&mut copy, row, col);

        let by_col = find_mirror(&copy, old_by_col);
        let by_row = find_mirror(&transpose(&copy), old_by_row);

        match (by_col, by_row) {
            (None, r @ Some(_)) => return (None, r),
            (c @ Some(_), None) => return (c, None),
            _ => {}
        }
        flip(&mut copy, row, col);
    }
    unreachable!()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<(usize, Vec<Row>)> = token_groups(input, "\n\n", None)
        .into_iter()
        .enumerate()
        .map(|(id, map)| {
            (
                id,
                map.into_iter()
                    .map(|row: String| row.bytes().collect())
                    .collect(),
            )
        })
        .collect();

    let s = Instant::now();

    let mut cols_all = vec![];
    let mut rows_all = vec![];
    for (_, map) in &input {
        cols_all.push(find_mirror(&map, None));
        rows_all.push(find_mirror(&transpose(&map), None));
    }

    let part1 = rows_all.iter().copied().flatten().sum::<usize>() * 100
        + cols_all.iter().copied().flatten().sum::<usize>();

    let results: Vec<(Option<usize>, Option<usize>)> = input
        .par_iter()
        .map(|(id, map)| find_smudge(&map, cols_all[*id], rows_all[*id]))
        .collect();
    let part2 = results
        .iter()
        .map(|(_, by_row)| *by_row)
        .flatten()
        .sum::<usize>()
        * 100
        + results
            .iter()
            .map(|(by_col, _)| *by_col)
            .flatten()
            .sum::<usize>();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(29165, part1);
        assert_eq!(32192, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
