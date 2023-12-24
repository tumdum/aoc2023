use anyhow::Result;
use std::{
    ops::{Add, Div, Mul, Sub},
    time::{Duration, Instant},
};
use z3::ast::Ast;
use z3::*;

use crate::input::token_groups;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct Pos2d {
    x: f64,
    y: f64,
}

impl std::fmt::Debug for Pos2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x, self.y)
    }
}

impl Pos2d {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add for Pos2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Pos2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Pos2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Pos2d> for f64 {
    type Output = Pos2d;

    fn mul(self, rhs: Pos2d) -> Self::Output {
        rhs * self
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Pos {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn to_2d(&self) -> Pos2d {
        Pos2d::new(self.x as f64, self.y as f64)
    }
}

impl Mul<i64> for Pos {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Pos> for i64 {
    type Output = Pos;

    fn mul(self, rhs: Pos) -> Self::Output {
        rhs * self
    }
}

impl Mul for Pos {
    type Output = i64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Div<i64> for Pos {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

fn cross(a: Pos2d, b: Pos2d) -> f64 {
    (a.x * b.y) - (a.y * b.x)
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Intersection {
    Point(Pos2d, f64, f64),
    Parallel,
    Colinear,
}

fn lines_intersect((p, r): (Pos2d, Pos2d), (q, s): (Pos2d, Pos2d)) -> Intersection {
    let eps = 0.001;

    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
    let denominator: f64 = cross(r, s);
    let numerator: f64 = cross(q - p, r);
    if denominator == 0.0 && numerator == 0.0 {
        Intersection::Colinear
    } else if denominator.abs() < eps && denominator.abs() > eps {
        Intersection::Parallel
    } else {
        let u = numerator / denominator;
        let t = cross(q - p, s) / denominator;

        Intersection::Point(p + t * r, t, u)
    }
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<(Pos, Pos)> = token_groups(&input.replace(",", " "), "\n", None)
        .into_iter()
        .map(|v| {
            let pos = Pos::new(v[0], v[1], v[2]);
            let speed = Pos::new(v[3], v[4], v[5]);
            (pos, speed)
        })
        .collect();

    let s = Instant::now();

    let test_area_x = (200000000000000f64, 400000000000000f64);
    let test_area_y = (200000000000000f64, 400000000000000f64);

    let mut part1 = 0u64;
    let mut max = (0, 0, 0);
    for i in 0..input.len() {
        max.0 = max.0.max(input[i].1.x.abs());
        max.1 = max.1.max(input[i].1.y.abs());
        max.2 = max.2.max(input[i].1.z.abs());
        for j in (i + 1)..input.len() {
            if i != j {
                match lines_intersect(
                    (input[i].0.to_2d(), input[i].1.to_2d()),
                    (input[j].0.to_2d(), input[j].1.to_2d()),
                ) {
                    Intersection::Point(p, t, u) => {
                        if p.x >= test_area_x.0
                            && p.x <= test_area_x.1
                            && p.y >= test_area_y.0
                            && p.y <= test_area_y.1
                            && t.is_sign_positive()
                            && u.is_sign_positive()
                        {
                            part1 += 1;
                        } else {
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let part2 = find_part2_solution(&input);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(15107, part1);
        assert_eq!(856642398547748, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn find_part2_solution(input: &[(Pos, Pos)]) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");
    let z = ast::Int::new_const(&ctx, "z");

    let dx = ast::Int::new_const(&ctx, "dx");
    let dy = ast::Int::new_const(&ctx, "dy");
    let dz = ast::Int::new_const(&ctx, "dz");

    for (id, (l1, d1)) in input.iter().enumerate() {
        let lx = ast::Int::from_i64(&ctx, l1.x);
        let ly = ast::Int::from_i64(&ctx, l1.y);
        let lz = ast::Int::from_i64(&ctx, l1.z);
        let ldx = ast::Int::from_i64(&ctx, d1.x);
        let ldy = ast::Int::from_i64(&ctx, d1.y);
        let ldz = ast::Int::from_i64(&ctx, d1.z);

        let t = ast::Int::new_const(&ctx, format!("t{id}"));

        let tmp_x = lx.add(ldx.mul(&t));
        let tmp_xx = (&x).add((&dx).mul(&t));
        solver.assert(&tmp_x._eq(&tmp_xx));

        let tmp_y = ly.add(ldy.mul(&t));
        let tmp_yy = (&y).add((&dy).mul(&t));
        solver.assert(&tmp_y._eq(&tmp_yy));

        let tmp_z = lz.add(ldz.mul(&t));
        let tmp_zz = (&z).add((&dz).mul(&t));
        solver.assert(&tmp_z._eq(&tmp_zz));
    }

    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();
    x + y + z
}
