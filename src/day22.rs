use anyhow::Result;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

use crate::input::token_groups;

const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Pos {
    vals: [i16; 3],
}

impl Pos {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { vals: [x, y, z] }
    }

    fn on_ground(&self) -> bool {
        self.vals[2] == 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    id: usize,
    points: FxHashSet<Pos>,
    min: Pos,
}

impl Brick {
    fn new(a: Pos, b: Pos, id: usize) -> Self {
        let mut points: FxHashSet<Pos> = Default::default();
        for x in a.vals[X]..=b.vals[X] {
            for y in a.vals[Y]..=b.vals[Y] {
                for z in a.vals[Z]..=b.vals[Z] {
                    points.insert(Pos::new(x, y, z));
                }
            }
        }
        let min = Self::compute_min(points.iter());
        Self { id, points, min }
    }

    fn compute_min<'a>(points: impl Iterator<Item = &'a Pos>) -> Pos {
        points.fold(Pos::new(i16::MAX, i16::MAX, i16::MAX), |a, b| {
            Pos::new(
                a.vals[0].min(b.vals[0]),
                a.vals[1].min(b.vals[1]),
                a.vals[2].min(b.vals[2]),
            )
        })
    }

    fn on_ground(&self) -> bool {
        self.points.iter().any(|p| p.on_ground())
    }

    fn min(&self, id: usize) -> i16 {
        self.min.vals[id]
    }

    fn lower(&self) -> Self {
        debug_assert!(self.min(Z) > 1);
        let points = self
            .points
            .iter()
            .cloned()
            .map(|mut p| {
                p.vals[Z] -= 1;
                p
            })
            .collect();
        let mut min = self.min.clone();
        min.vals[Z] -= 1;
        Self {
            id: self.id,
            points,
            min,
        }
    }
}

fn fall<'a>(input: impl Iterator<Item = &'a Brick>) -> Vec<Brick> {
    let mut settled: FxHashSet<Pos> = Default::default();
    let mut final_bricks = vec![];

    for b in input {
        let mut current = b.clone();
        loop {
            if current.on_ground() {
                break;
            }
            let lower = current.lower();
            if settled.intersection(&lower.points).next().is_some() {
                break;
            }
            current = lower;
        }
        for p in &current.points {
            settled.insert(*p);
        }
        final_bricks.push(current);
    }

    final_bricks
}

fn how_many_will_fall(
    id1: usize,
    final_bricks: &[Brick],
    final_bricks_map: &FxHashMap<usize, &FxHashSet<Pos>>,
) -> Option<i16> {
    let mut can_rem = true;
    for id2 in 0..final_bricks.len() {
        if id1 == id2 || final_bricks[id2].on_ground() {
            continue;
        }
        if final_bricks[id2].min(Z) <= final_bricks[id1].min(Z) {
            continue;
        }

        let lower = final_bricks[id2].lower();
        let can_lower_id2 = final_bricks
            .iter()
            .filter(|b| b.id != final_bricks[id1].id && b.id != final_bricks[id2].id)
            .all(|b| b.points.intersection(&lower.points).next().is_none());

        if can_lower_id2 {
            can_rem = false;
            break;
        }
    }
    if can_rem {
        return None;
    } else {
        let mut ret = 0;
        let falled_bricks = fall(final_bricks.iter().filter(|b| b.id != final_bricks[id1].id));
        for b in falled_bricks {
            if b.points != **final_bricks_map.get(&b.id).unwrap() {
                ret += 1;
            }
        }
        Some(ret)
    }
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input = input.replace(",", " ").replace("~", " ");

    let s = Instant::now();

    let mut input: Vec<Brick> = token_groups(&input, "\n", None)
        .into_iter()
        .enumerate()
        .map(|(id, v)| Brick::new(Pos::new(v[0], v[1], v[2]), Pos::new(v[3], v[4], v[5]), id))
        .collect();
    input.sort_unstable_by_key(|b| b.min(Z));

    let bricks = fall(input.iter());

    let mut bricks_map: FxHashMap<usize, &FxHashSet<Pos>> = Default::default();
    for b in &bricks {
        bricks_map.insert(b.id, &b.points);
    }

    let will_fall: Vec<i16> = (0..bricks.len())
        .into_par_iter()
        .flat_map(|id1| how_many_will_fall(id1, &bricks, &bricks_map))
        .collect();

    let part1 = bricks.len() - will_fall.len() as usize;
    let part2 = will_fall.into_iter().map(|v| v as i32).sum::<i32>();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(475, part1);
        assert_eq!(79144, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
