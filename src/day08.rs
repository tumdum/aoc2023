use anyhow::Result;
use std::time::{Duration, Instant};

use crate::input::tokens;

type Node = u16;

fn to_node(chars: &str) -> Node {
    chars
        .chars()
        .map(|c| c as u8)
        .fold(0, |a, b| a << 5 | (b - b'A') as u16)
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input = input.replace("=", " ");
    let input = input.replace("(", " ");
    let input = input.replace(")", " ");
    let input = input.replace(",", " ");
    let input: Vec<String> = tokens(&input, None).into_iter().collect();
    let dirs = input[0].clone();
    let input: Vec<String> = input.into_iter().skip(1).collect();
    let keys: Vec<Node> = input.chunks(3).map(|c| to_node(&c[0])).collect();
    let max = *keys.iter().max().unwrap() + 1;
    let mut input2: Vec<(Node, Node)> = vec![(0, 0); max as usize];
    input.chunks(3).for_each(|c| {
        input2[to_node(&c[0]) as usize] = (to_node(&c[1]), to_node(&c[2]));
    });

    let s = Instant::now();
    let mut current = to_node("AAA");
    let mut part1 = 0u64;
    for dir in dirs.chars().cycle() {
        if current == 0x6739 {
            break;
        }
        part1 += 1;
        if dir == 'R' {
            current = input2[current as usize].1;
        } else {
            current = input2[current as usize].0;
        }
    }

    let mut ghost_current: Vec<(usize, Node)> = keys
        .iter()
        .filter(|s| (*s & 0b00011111) == 25)
        .copied()
        .enumerate()
        .collect();
    let mut ends: Vec<Option<u64>> = vec![None; ghost_current.len()];

    let mut cycle_lens = vec![];
    for (steps, dir) in dirs.chars().cycle().enumerate() {
        if let Some(idx) = ghost_current
            .iter()
            .position(|(_, s)| (s & 0b00011111) == 25)
        {
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
                ghost_current[i].1 = input2[ghost_current[i].1 as usize].1;
            }
        } else {
            for i in 0..ghost_current.len() {
                ghost_current[i].1 = input2[ghost_current[i].1 as usize].0;
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
