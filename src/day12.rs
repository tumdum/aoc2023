use anyhow::Result;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::input::token_groups;

fn solve_one(input: &[char], groups: &[i8], ip: usize, ig: usize) -> usize {
    solve_cached(input, groups, ip, ig, &mut Default::default())
}

fn solve_cached(
    input: &[char],
    groups: &[i8],
    ip: usize,
    ig: usize,
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if let Some(ret) = cache.get(&(ip, ig)) {
        return *ret;
    }
    let ret = solve_aux(input, groups, ip, ig, cache);
    cache.insert((ip, ig), ret);
    ret
}

fn solve_aux(
    input: &[char],
    groups: &[i8],
    ip: usize,
    ig: usize,
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if input[ip..].is_empty() && !groups[ig..].is_empty() {
        return 0;
    }
    if input[ip..].is_empty() && groups[ig..].is_empty() {
        return 1;
    }
    if groups[ig..].is_empty() {
        if input[ip..].iter().all(|c| *c != '#') {
            return 1;
        } else {
            return 0;
        }
    }
    let current_i = input[ip];
    fn solve_hash(
        input: &[char],
        groups: &[i8],
        ip: usize,
        ig: usize,
        cache: &mut FxHashMap<(usize, usize), usize>,
    ) -> usize {
        {
            // next groups[ig] chars need to be '#' or '?' and next one needs to
            // be either '.' or '?' or end of str.
            let next_hash_or_any = groups[ig] as usize;
            if (ip + next_hash_or_any) > input.len() {
                return 0;
            }
            if input[ip..]
                .iter()
                .take(next_hash_or_any)
                .all(|c| *c == '#' || *c == '?')
            {
                let next = input.get(ip + next_hash_or_any).copied();
                if next != Some('#') {
                    if next.is_none() {
                        if groups[ig + 1..].is_empty() {
                            return 1;
                        }
                    } else {
                        return solve_cached(
                            input,
                            groups,
                            ip + next_hash_or_any + 1,
                            ig + 1,
                            cache,
                        );
                    }
                }
            }
            return 0;
        }
    }
    if current_i == '.' {
        return solve_cached(input, groups, ip + 1, ig, cache);
    } else if current_i == '#' {
        return solve_hash(input, groups, ip, ig, cache);
    } else if current_i == '?' {
        return solve_cached(input, groups, ip + 1, ig, cache)
            + solve_hash(input, groups, ip, ig, cache);
    }

    unreachable!()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<(Vec<char>, Vec<i8>)> = token_groups(input, "\n", None)
        .into_iter()
        .map(|v: Vec<String>| {
            (
                v[0].chars().collect(),
                v[1].split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect();
    let s = Instant::now();

    let part1 = input
        .iter()
        .map(|(input, target)| solve_one(&input, &target, 0, 0) as i64)
        .sum::<i64>();

    let part2 = input
        .iter()
        .map(|(input, target)| {
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
            solve_one(&i, &t, 0, 0) as i64
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
