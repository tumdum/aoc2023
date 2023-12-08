use anyhow::Result;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::input::tokens;

type Node = [u8; 3];

fn to_node(chars: &str) -> Node {
    chars
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input = input.replace("=", " ");
    let input = input.replace("(", " ");
    let input = input.replace(")", " ");
    let input = input.replace(",", " ");
    let input: Vec<String> = tokens(&input, None).into_iter().collect();
    let dirs = input[0].clone();
    let input: Vec<String> = input.into_iter().skip(1).collect();
    let input: FxHashMap<Node, (Node, Node)> = input
        .chunks(3)
        .map(|c| (to_node(&c[0]), (to_node(&c[1]), to_node(&c[2]))))
        .collect();

    let s = Instant::now();
    let mut current = to_node("AAA");
    let mut part1 = 0u64;
    for dir in dirs.chars().cycle() {
        if current == [b'Z', b'Z', b'Z'] {
            break;
        }
        part1 += 1;
        if dir == 'R' {
            current = input.get(&current).unwrap().1.clone();
        } else {
            current = input.get(&current).unwrap().0.clone();
        }
    }

    let mut ghost_current: Vec<(usize, Node)> = input
        .keys()
        .filter(|s| s[2] == b'A')
        .cloned()
        .enumerate()
        .collect();
    let mut ends: Vec<Option<u64>> = vec![None; ghost_current.len()];

    let mut cycle_lens = vec![];
    for (steps, dir) in dirs.chars().cycle().enumerate() {
        if let Some(idx) = ghost_current.iter().position(|(_, s)| s[2] == b'Z') {
            let id = ghost_current[idx].0;
            if let Some(previous_end) = ends[id] {
                cycle_lens.push(steps as u64 - previous_end);
                ghost_current.remove(idx);
            } else {
                ends[id] = Some(steps as u64);
            }
        }

        if ghost_current.is_empty() {
            break;
        }

        if dir == 'R' {
            for i in 0..ghost_current.len() {
                ghost_current[i].1 = input.get(&ghost_current[i].1).unwrap().1.clone();
            }
        } else {
            for i in 0..ghost_current.len() {
                ghost_current[i].1 = input.get(&ghost_current[i].1).unwrap().0.clone();
            }
        }
    }
    let part2 = cycle_lens.iter().fold(1, |l, r| num::integer::lcm(l, *r));

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(20093, part1);
        assert_eq!(22103062509257, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
