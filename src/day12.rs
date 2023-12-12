use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use smallvec::{SmallVec, ToSmallVec};
use std::time::{Duration, Instant};
use tikv_jemallocator::Jemalloc;

use crate::input::token_groups;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

type Target = [i8];
type Input = SmallVec<[u8; 36]>;

fn solve_rec<'a>(
    i_r: (usize, usize),
    cache: &mut FxHashMap<(Input, &'a Target), i64>,
    input: &mut [u8],
    target: &'a [i8],
    t_r: (usize, usize),
) -> i64 {
    if target[t_r.0..t_r.1].len() > input[i_r.0..i_r.1].len() {
        return 0;
    }

    let all_dot = input[i_r.0..i_r.1].iter().all(|c| *c == b'.');
    if all_dot && target[t_r.0..t_r.1].is_empty() {
        return 1;
    }
    let all_hash = input[i_r.0..i_r.1].iter().all(|c| *c == b'#');
    if all_hash
        && target[t_r.0..t_r.1].len() == 1
        && target[t_r.0] as usize == input[i_r.0..i_r.1].len()
    {
        return 1;
    }
    if let Some(ret) = cache.get(&(input[i_r.0..i_r.1].to_smallvec(), &target[t_r.0..t_r.1])) {
        return *ret;
    }

    let first_unknown = input[i_r.0..i_r.1].iter().position(|c| *c == b'?');

    let point = if let Some(p) = first_unknown {
        p
    } else {
        let g = input[i_r.0..i_r.1].iter().group_by(|c| **c);
        let got_groups = g.into_iter().filter(|(c, _)| *c == b'#').map(|(_, g)| {
            let tmp: i8 = g.into_iter().count().try_into().unwrap();
            tmp
        });
        if got_groups.eq(target[t_r.0..t_r.1].iter().copied()) {
            return 1;
        } else {
            return 0;
        }
    };
    let point_orig = i_r.0 + point;
    let g = input[i_r.0..point_orig].iter().group_by(|c| **c);
    let got_groups: SmallVec<[i8; 20]> = g
        .into_iter()
        .filter(|(c, _)| *c == b'#')
        .map(|(_, g)| {
            let tmp: i8 = g.into_iter().count().try_into().unwrap();
            tmp
        })
        .collect();

    let ret = {
        input[point_orig] = b'#';

        let solution_with_hash = solve_rec(i_r, cache, input, target, t_r);

        input[point_orig] = b'.';

        let mut solutions_with_dot: SmallVec<[i64; 20]> = Default::default();
        if target[t_r.0..t_r.1].starts_with(&got_groups) {
            for target_split in 0..=target[t_r.0..t_r.1].len() {
                solutions_with_dot.push(
                    solve_rec(
                        (i_r.0, point_orig),
                        cache,
                        input,
                        target,
                        (t_r.0, t_r.0 + target_split),
                    ) * solve_rec(
                        (point_orig, i_r.1),
                        cache,
                        input,
                        target,
                        (t_r.0 + target_split, t_r.1),
                    ),
                )
            }
        }

        let dot_solutions = solutions_with_dot.iter().copied().sum::<i64>();

        solution_with_hash + dot_solutions
    };

    input[point_orig] = b'?';
    cache.insert(
        (input[i_r.0..i_r.1].to_smallvec(), &target[t_r.0..t_r.1]),
        ret,
    );

    ret
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<(Vec<char>, Vec<i8>, usize)> = token_groups(input, "\n", None)
        .into_iter()
        .enumerate()
        .map(|(i, v): (usize, Vec<String>)| {
            (
                v[0].chars().collect(),
                v[1].split(',').map(|n| n.parse().unwrap()).collect(),
                i,
            )
        })
        .collect();

    let mut cache: FxHashMap<(Input, &Target), i64> = Default::default();
    let part1 = input
        .iter()
        .map(|(input, target, _)| {
            solve_rec(
                (0, input.len()),
                &mut cache,
                &mut input.iter().map(|c| *c as u8).collect_vec(),
                target,
                (0, target.len()),
            )
        })
        .sum::<i64>();

    let s = Instant::now();

    let part2 = input
        .par_iter()
        .map(|(input, target, _id)| {
            let mut cache: FxHashMap<(Input, &Target), i64> = Default::default();

            let mut i = input.to_vec();
            i.push('?');
            i.extend_from_slice(&input);
            i.push('?');
            i.extend_from_slice(&input);
            i.push('?');
            i.extend_from_slice(&input);
            i.push('?');
            i.extend_from_slice(&input);

            let mut t = target.to_vec();
            t.extend_from_slice(&target);
            t.extend_from_slice(&target);
            t.extend_from_slice(&target);
            t.extend_from_slice(&target);
            // let ret = find_solution(&i, &t, 0, 0, 0);
            let mut input_copy: Input = i.iter().copied().map(|c| c as u8).collect();
            solve_rec((0, i.len()), &mut cache, &mut input_copy, &t, (0, t.len()))
        })
        .sum::<i64>();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(7670, part1);
        assert_eq!(157383940585037, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
