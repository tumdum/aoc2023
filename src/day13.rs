use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;
use std::time::{Duration, Instant};

use crate::{input::token_groups, vec::transpose};

fn find_mirror(m: &[Vec<char>], not_this: Option<usize>) -> Option<usize> {
    let mut longest = None;
    for col in 0..m[0].len() {
        let mut mirrored_rows = 0;
        for row in 0..m.len() {
            let mut left = m[row][..col].to_vec();
            if left.is_empty() {
                continue;
            }
            left.reverse();
            let right = &m[row][col..];
            if right.is_empty() {
                continue;
            }
            if left.len() >= right.len() {
                if left.starts_with(&right) {
                    mirrored_rows += 1;
                }
            } else {
                if right.starts_with(&left) {
                    mirrored_rows += 1;
                }
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

fn flip(m: &mut [Vec<char>], row: usize, col: usize) {
    let v = m[row][col];
    let new = if v == '.' { '#' } else { '.' };
    m[row][col] = new;
}

fn find_smudge(
    map: &[Vec<char>],
    old_by_col: Option<usize>,
    old_by_row: Option<usize>,
) -> (Option<usize>, Option<usize>) {
    let mut new_by_col = None;
    let mut new_by_row = None;

    'outer: for (row, col) in iproduct!(0..map.len(), 0..map[0].len()) {
        let mut copy = map.to_vec();
        flip(&mut copy, row, col);

        let by_col = find_mirror(&copy, old_by_col);
        let by_row = find_mirror(&transpose(copy), old_by_row);

        match (by_col, by_row) {
            (None, Some(by_row)) => {
                new_by_row = Some(by_row);
                break 'outer;
            }
            (Some(by_col), None) => {
                new_by_col = Some(by_col);
                break 'outer;
            }
            _ => {}
        }
    }
    (new_by_col, new_by_row)
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<Vec<char>>> = token_groups(input, "\n\n", None)
        .into_iter()
        .map(|map| {
            map.into_iter()
                .map(|row: String| row.chars().collect())
                .collect()
        })
        .collect();

    let s = Instant::now();

    let mut cols_all = vec![];
    let mut rows_all = vec![];
    for map in &input {
        cols_all.push(find_mirror(&map, None));
        rows_all.push(find_mirror(&transpose(map.to_vec()), None));
    }

    let part1 = rows_all.iter().copied().flatten().sum::<usize>() * 100
        + cols_all.iter().copied().flatten().sum::<usize>();

    let results: Vec<(Option<usize>, Option<usize>)> = input
        .into_iter()
        .enumerate()
        .par_bridge()
        .map(|(id, map)| find_smudge(&map, cols_all[id], rows_all[id]))
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
