use anyhow::Result;
use itertools::Itertools;
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use crate::input::tokens;

type Pos = crate::pos::Pos<i32>;
type Dir2d = crate::pos::Pos<i8>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
    U,
    D,
    R,
    L,
}

impl From<Dir> for Dir2d {
    fn from(value: Dir) -> Self {
        match value {
            Dir::U => Dir2d::new(0, -1),
            Dir::D => Dir2d::new(0, 1),
            Dir::R => Dir2d::new(1, 0),
            Dir::L => Dir2d::new(-1, 0),
        }
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s.chars().next() {
            Some('U') => Self::U,
            Some('D') => Self::D,
            Some('L') => Self::L,
            Some('R') => Self::R,
            c => todo!("c: '{c:?}"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Op {
    dir: Dir,
    dist: i32,
    color: String,
}

impl Op {
    fn real_color(&self) -> (i32, Dir2d) {
        let dist: String = self.color.chars().take(5).collect();
        let dist = i32::from_str_radix(&dist, 16).unwrap();
        let dir = match self.color.chars().last().unwrap() {
            '0' => Dir::R,
            '1' => Dir::D,
            '2' => Dir::L,
            '3' => Dir::U,
            c => todo!("c: {c}"),
        };
        (dist, dir.into())
    }
}

fn shoelace_formula(corners: &[Pos]) -> i64 {
    let mut total = 0i64;
    for idxs in (0..corners.len()).chain(0..1).collect_vec().windows(2) {
        // x1 * y2 - x2 * y1
        total += corners[idxs[0]].x as i64 * corners[idxs[1]].y as i64
            - corners[idxs[1]].x as i64 * corners[idxs[0]].y as i64;
    }
    total / 2
}

fn border_len(corners: &[Pos]) -> i64 {
    let mut total = 0;
    for pair in corners.windows(2) {
        total += pair[0].manhattan_dist(&pair[1]) as i64;
    }
    total + corners[0].manhattan_dist(corners.last().unwrap()) as i64
}

fn picks_theorem(corners: &[Pos]) -> i64 {
    shoelace_formula(corners) + border_len(&corners) / 2 + 1
}

fn find_corners(input: &[Op], part2: bool) -> Vec<Pos> {
    let mut curr = Pos::new(0, 0);
    let mut corners: Vec<Pos> = Default::default();

    for op in input.iter() {
        let (dist, dir): (i32, Dir2d) = if !part2 {
            (op.dist, op.dir.into())
        } else {
            op.real_color()
        };

        curr = curr + Pos::new(dir.x as i32, dir.y as i32) * dist;
        corners.push(curr);
    }

    corners
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Op> = tokens(input, None)
        .chunks(3)
        .map(|chunk: &[String]| {
            let dir = chunk[0].parse().unwrap();
            let dist = chunk[1].parse().unwrap();
            let l = chunk[2].len();
            let color: String = chunk[2].chars().skip(2).take(l - 3).collect();

            Op { dir, dist, color }
        })
        .collect();

    let s = Instant::now();

    let corners = find_corners(&input, false);
    let part1 = picks_theorem(&corners);
    let corners = find_corners(&input, true);
    let part2 = picks_theorem(&corners);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(47527, part1);
        assert_eq!(52240187443190, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

#[test]
fn works_for_example() {
    assert_eq!(
        4,
        picks_theorem(&[
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(1, 1),
            Pos::new(0, 1),
        ])
    );
    assert_eq!(
        9,
        picks_theorem(&[
            Pos::new(0, 0),
            Pos::new(2, 0),
            Pos::new(2, 2),
            Pos::new(0, 2),
        ])
    );
    assert_eq!(
        62,
        picks_theorem(&[
            Pos { x: 6, y: 0 },
            Pos { x: 6, y: 5 },
            Pos { x: 4, y: 5 },
            Pos { x: 4, y: 7 },
            Pos { x: 6, y: 7 },
            Pos { x: 6, y: 9 },
            Pos { x: 1, y: 9 },
            Pos { x: 1, y: 7 },
            Pos { x: 0, y: 7 },
            Pos { x: 0, y: 5 },
            Pos { x: 2, y: 5 },
            Pos { x: 2, y: 2 },
            Pos { x: 0, y: 2 },
            Pos { x: 0, y: 0 }
        ])
    );
    assert_eq!(
        38,
        border_len(&[
            Pos { x: 6, y: 0 },
            Pos { x: 6, y: 5 },
            Pos { x: 4, y: 5 },
            Pos { x: 4, y: 7 },
            Pos { x: 6, y: 7 },
            Pos { x: 6, y: 9 },
            Pos { x: 1, y: 9 },
            Pos { x: 1, y: 7 },
            Pos { x: 0, y: 7 },
            Pos { x: 0, y: 5 },
            Pos { x: 2, y: 5 },
            Pos { x: 2, y: 2 },
            Pos { x: 0, y: 2 },
            Pos { x: 0, y: 0 }
        ])
    );
}
