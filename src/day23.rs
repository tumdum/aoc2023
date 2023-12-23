use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    cmp::Reverse,
    collections::VecDeque,
    ops::Add,
    time::{Duration, Instant},
};

use crate::{dijkstra::dijkstra, input::tokens, vec::StrVec};

type Pos = crate::pos::Pos<i16>;

fn get(input: &[StrVec], p: Pos) -> Option<u8> {
    input
        .get(p.y as usize)
        .and_then(|r| r.get(p.x as usize))
        .copied()
}

#[derive(Debug, PartialEq, Eq, Clone, Ord, Hash, Default)]
struct Len(usize);

impl Add for Len {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Len(self.0 + rhs.0)
    }
}

impl PartialOrd for Len {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.0).partial_cmp(&Reverse(other.0))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
struct Position {
    current: Pos,
    previous: Vec<u16>,
}

impl Position {
    fn new(current: Pos) -> Self {
        Self {
            current,
            previous: Default::default(),
        }
    }

    fn len(&self, w: i16) -> Len {
        debug_assert!(
            !self.previous.contains(&(self.current.idx_1d(w) as u16)),
            "self: {self:?}"
        );
        Len(1)
    }

    fn neighbours(&self, input: &[StrVec]) -> Vec<Self> {
        let w = input[0].len() as i16;

        let next = match input[self.current.y as usize][self.current.x as usize] {
            b'>' => Some(Pos::new(self.current.x + 1, self.current.y)),
            b'<' => Some(Pos::new(self.current.x - 1, self.current.y)),
            b'^' => Some(Pos::new(self.current.x, self.current.y - 1)),
            b'v' => Some(Pos::new(self.current.x, self.current.y + 1)),
            _ => None,
        };
        if let Some(p) = next {
            if !self.seen(p, w) {
                debug_assert!(get(input, p).unwrap() != b'#');
                return vec![self.move_to(p, w)];
            } else {
                return vec![];
            }
        }

        let mut ret = vec![];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = Pos::new(self.current.x + dx, self.current.y + dy);
            if let Some(v) = get(input, next) {
                if v != b'#' && !self.seen(next, w) {
                    ret.push(self.move_to(next, w));
                }
            }
        }
        ret
    }

    fn move_to(&self, next: Pos, w: i16) -> Position {
        debug_assert!(!self.seen(next, w));
        let mut previous = self.previous.clone();
        previous.push(self.current.idx_1d(w) as u16);
        Self {
            current: next,
            previous,
        }
    }

    fn seen(&self, pos: Pos, w: i16) -> bool {
        self.previous.contains(&(pos.idx_1d(w) as u16)) || self.current == pos
    }
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<StrVec> = tokens(input, None);

    let s = Instant::now();

    let w = input[0].len() as i16;

    let start = Pos::new(
        input[0]
            .iter()
            .enumerate()
            .find(|x| x.1 == &b'.')
            .map(|x| x.0)
            .unwrap() as i16,
        0,
    );
    let end = Pos::new(
        input[input.len() - 1]
            .iter()
            .enumerate()
            .find(|x| x.1 == &b'.')
            .map(|x| x.0)
            .unwrap() as i16,
        (input.len() - 1) as i16,
    );

    let start = Position::new(start);

    let neighbours = |p: &Position| -> Vec<(Position, Len)> {
        p.neighbours(&input)
            .into_iter()
            .map(|p| {
                let l = p.len(w);
                (p, l)
            })
            .collect()
    };

    let (costs, _prevs) = dijkstra(&[start.clone()], neighbours);
    let mut part1 = 0;
    for (p, c) in costs {
        if p.current == end {
            part1 = part1.max(c.0);
        }
    }

    let (g, dist) = compress(&input, start.current, end);

    let mut todo: VecDeque<(Pos, Vec<Pos>, usize)> = Default::default();
    todo.push_back((start.current.clone(), Default::default(), 0));

    let mut part2 = 0;
    while let Some((next, seen, d)) = todo.pop_front() {
        debug_assert!(!seen.contains(&next));
        if next == end {
            part2 = part2.max(d);
            continue;
        }
        for candidate in g.get(&next).unwrap() {
            debug_assert_ne!(next, *candidate);
            if seen.contains(&candidate) {
                continue;
            }
            let mut seen = seen.clone();
            seen.push(next);
            let part_d = dist.get(&(next, *candidate)).unwrap();
            todo.push_back((*candidate, seen, d + part_d));
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(2402, part1);
        assert_eq!(6450, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

type G = FxHashMap<Pos, FxHashSet<Pos>>;

fn compress(input: &[StrVec], start: Pos, end: Pos) -> (G, FxHashMap<(Pos, Pos), usize>) {
    let mut graph: FxHashMap<Pos, FxHashSet<Pos>> = Default::default();
    let mut distances: FxHashMap<(Pos, Pos), usize> = Default::default();
    let mut todo: VecDeque<(Pos, Pos, Pos, usize)> = Default::default();
    let mut seen: FxHashSet<Pos> = Default::default();

    todo.push_back((start, start, start, 0));
    seen.insert(start);

    while let Some((orig, prev, next, dist_to_next)) = todo.pop_front() {
        let dist = dist_to_next + 1;
        if next == end {
            graph.entry(orig).or_default().insert(next);
            graph.entry(next).or_default().insert(orig);
            distances.insert((orig, next), dist_to_next);
            distances.insert((next, orig), dist_to_next);

            continue;
        }
        let neighbours = neighbours_of(input, next, prev);

        if neighbours.len() == 1 {
            todo.push_back((orig, next, neighbours[0], dist));
        } else {
            graph.entry(orig).or_default().insert(next);
            graph.entry(next).or_default().insert(orig);
            distances.insert((orig, next), dist_to_next);
            distances.insert((next, orig), dist_to_next);
            if !seen.contains(&next) {
                seen.insert(next);
                for n in neighbours {
                    todo.push_back((next, next, n, 1));
                }
            }
        }
    }

    (graph, distances)
}

fn neighbours_of(input: &[StrVec], current: Pos, prev: Pos) -> Vec<Pos> {
    let mut ret = vec![];
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbour = Pos::new(current.x + dx, current.y + dy);
        if neighbour != prev {
            if let Some(v) = get(input, neighbour) {
                if v != b'#' {
                    ret.push(neighbour);
                }
            }
        }
    }
    ret
}
