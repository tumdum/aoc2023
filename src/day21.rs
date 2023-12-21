use anyhow::Result;
use itertools::iproduct;
use rustc_hash::FxHashSet;
use smallvec::{smallvec, SmallVec};
use std::time::{Duration, Instant};

type Pos = crate::pos::Pos<i16>;

use crate::{input::tokens, vec::StrVec};

fn real_coord(val: i16, len: i16) -> i16 {
    if val >= 0 {
        val % len
    } else {
        (len + val % len) % len
    }
}

fn real_pos(pos: Pos, w: i16, h: i16) -> Pos {
    let x = real_coord(pos.x, w);
    let y = real_coord(pos.y, h);
    Pos::new(x, y)
}

fn get(input: &[StrVec], pos: Pos) -> Option<u8> {
    let h = input.len() as i16;
    let w = input[0].len() as i16;
    let pos_new = real_pos(pos, w, h);
    input
        .get(pos_new.y as usize)
        .and_then(|row| row.get(pos_new.x as usize))
        .copied()
}

fn neighbours(input: &[StrVec], pos: Pos) -> SmallVec<[(Pos, u8); 4]> {
    let mut ret = smallvec![];
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let p = Pos::new(pos.x + dx, pos.y + dy);
        if let Some(v) = get(input, p) {
            ret.push((p, v));
        }
    }

    ret
}

fn travel<'a>(
    input: &[StrVec],
    start: impl Iterator<Item = &'a Pos>,
    steps_to_capture: &[usize],
) -> Vec<i64> {
    let mut sizes = vec![];
    let mut current: FxHashSet<Pos> = start.copied().collect();
    let steps = *steps_to_capture.last().unwrap() as usize + 1;
    for i in 0..steps {
        if steps_to_capture.contains(&i) {
            sizes.push(current.len() as i64);
        }
        current = current
            .iter()
            .flat_map(|p| neighbours(input, *p))
            .filter(|(_, val)| *val != b'#')
            .map(|(p, _)| p)
            .collect();
    }
    sizes
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<StrVec> = tokens(input, None);

    let s = Instant::now();

    let h = input.len() as i16;
    let w = input[0].len() as i16;

    let start = iproduct!(0..w, 0..h)
        .flat_map(|(x, y)| {
            if input[y as usize][x as usize] == b'S' {
                Some(Pos::new(x, y))
            } else {
                None
            }
        })
        .next()
        .unwrap();
    let steps_to_capture = [64, 65, 65 + 131, 65 + 131 * 2];
    let sizes = travel(&input, [start].iter(), &steps_to_capture);
    let part1 = sizes[0];
    let ys = [sizes[1] as f64, sizes[2] as f64, sizes[3] as f64];

    let params: Vec<i64> = polyfit_rs::polyfit_rs::polyfit(&[0f64, 1f64, 2f64], &ys, 2)
        .unwrap()
        .iter()
        .map(|v| v.round() as i64)
        .collect();

    let target = 26501365;
    assert_eq!(0, (target - 65) % 131);
    let target = (target - 65) / 131;
    let part2 = params[2] * target * target + params[1] * target + params[0];

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(3746, part1);
        assert_eq!(623540829615589, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }

    Ok(e)
}
