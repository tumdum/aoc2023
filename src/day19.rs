use anyhow::Result;
use rustc_hash::FxHashMap;
use std::{
    fmt::Debug,
    str::FromStr,
    time::{Duration, Instant},
};

use crate::input::{token_groups, tokens};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }

    fn is_accepted(&self, rules: &FxHashMap<String, Vec<Rule>>) -> bool {
        let mut curr = "in".to_owned();
        loop {
            // println!("curr: {curr}");
            let rules = rules.get(&curr).unwrap();
            for rule in rules {
                match rule.eval(self) {
                    Res::Accept => return true,
                    Res::Reject => return false,
                    Res::Continue => {}
                    Res::Jump(target) => {
                        curr = target;
                        break;
                    }
                }
            }
        }
        return false;
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.replace(|c: char| !c.is_ascii_digit(), " ");
        let n: Vec<i64> = tokens(&s, None);
        Ok(Part {
            x: n[0],
            m: n[1],
            a: n[2],
            s: n[3],
        })
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Relation {
    name: char,
    op: char,
    constant: i64,
    negation: bool,
}
impl Relation {
    fn apply(&self, r: &mut PossibleRange) {
        let range: &mut (i64, i64) = match self.name {
            'x' => &mut r.x,
            'm' => &mut r.m,
            'a' => &mut r.a,
            's' => &mut r.s,
            _ => todo!(),
        };
        match (self.op, self.negation) {
            ('<', true) => {
                range.0 = self.constant.max(range.0);
            }
            ('<', false) => {
                range.1 = (self.constant - 1).min(range.1);
            }
            ('>', true) => {
                range.1 = self.constant.min(range.1);
            }
            ('>', false) => {
                range.0 = (self.constant + 1).max(range.0);
            }
            other => todo!("other: {other:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Rule {
    Relation {
        name: char,
        op: char,
        constant: i64,
        target: String,
    },
    Jump(String),
    Accept,
    Reject,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Res {
    Accept,
    Reject,
    Continue,
    Jump(String),
}

impl Rule {
    fn eval(&self, part: &Part) -> Res {
        match self {
            Self::Accept => Res::Accept,
            Self::Reject => Res::Reject,
            Self::Relation {
                name,
                op,
                constant,
                target,
            } => {
                let val = match name {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    other => todo!("name: {other}"),
                };
                let result = if *op == '<' {
                    val < *constant
                } else {
                    val > *constant
                };
                if !result {
                    return Res::Continue;
                } else {
                    if target == "A" {
                        return Res::Accept;
                    } else if target == "R" {
                        return Res::Reject;
                    } else {
                        return Res::Jump(target.clone());
                    }
                }
            }
            Rule::Jump(target) => Res::Jump(target.clone()),
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(idx) = s.chars().position(|c| c == ':') {
            let expr: String = s.chars().take(idx).collect();
            let target: String = s.chars().skip(idx + 1).collect();
            let op = expr.chars().find(|c| *c == '<' || *c == '>').unwrap();
            let mut s = expr.split(|c| c == '<' || c == '>');
            let name = s.next().unwrap().chars().next().unwrap();
            let constant: i64 = s.next().unwrap().parse().unwrap();
            return Ok(Self::Relation {
                name,
                op,
                constant,
                target,
            });
        } else {
            if s == "A" {
                return Ok(Self::Accept);
            } else if s == "R" {
                return Ok(Self::Reject);
            } else if s.chars().all(|c| c.is_lowercase()) {
                return Ok(Self::Jump(s.to_owned()));
            }
        }
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Location {
    rule: String,
    next_offset: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PossibleRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PossibleRange {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn size(&self) -> i64 {
        let x = self.x.1 - self.x.0 + 1;
        let m = self.m.1 - self.m.0 + 1;
        let a = self.a.1 - self.a.0 + 1;
        let s = self.s.1 - self.s.0 + 1;
        x * m * a * s
    }
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<String>> = token_groups(input, "\n\n", Some("\n"));

    let parts: Vec<Part> = input[1].iter().map(|l| l.parse().unwrap()).collect();
    let mut all_rules: FxHashMap<String, Vec<Rule>> = Default::default();
    for line in &input[0] {
        let open = line.chars().position(|c| c == '{').unwrap();
        let name: String = line.chars().take(open).collect();
        let rules: String = line
            .chars()
            .skip(open + 1)
            .take_while(|c| *c != '}')
            .collect();
        let rules: Vec<Rule> = rules.split(',').map(|s| s.parse().unwrap()).collect();
        all_rules.insert(name, rules);
    }

    let s = Instant::now();

    let part1 = parts
        .iter()
        .filter(|part| part.is_accepted(&all_rules))
        .map(|part| part.sum())
        .sum::<i64>();

    let mut states: Vec<(Location, Vec<Relation>)> = vec![];
    let mut accepted: Vec<Vec<Relation>> = vec![];

    states.push((
        Location {
            rule: "in".to_owned(),
            next_offset: 0,
        },
        vec![],
    ));

    while !states.is_empty() {
        let mut new_states: Vec<(Location, Vec<Relation>)> = vec![];

        for state in &states {
            match &all_rules.get(&state.0.rule).unwrap()[state.0.next_offset] {
                Rule::Relation {
                    name,
                    op,
                    constant,
                    target,
                } => {
                    let mut rule = Relation {
                        name: *name,
                        op: *op,
                        constant: *constant,
                        negation: false,
                    };
                    let mut s = state.1.clone();
                    s.push(rule.clone());
                    if target == "A" {
                        accepted.push(s);
                    } else if target != "R" {
                        new_states.push((
                            Location {
                                rule: target.clone(),
                                next_offset: 0,
                            },
                            s,
                        ));
                    }

                    rule.negation = true;
                    let mut s = state.1.clone();
                    s.push(rule.clone());
                    new_states.push((
                        Location {
                            rule: state.0.rule.clone(),
                            next_offset: state.0.next_offset + 1,
                        },
                        s,
                    ));
                }
                Rule::Jump(target) => {
                    new_states.push((
                        Location {
                            rule: target.clone(),
                            next_offset: 0,
                        },
                        state.1.clone(),
                    ));
                }
                Rule::Accept => {
                    accepted.push(state.1.clone());
                }
                Rule::Reject => {}
            }
        }

        states = new_states;
    }

    let mut part2 = 0;
    for rules in &accepted {
        let mut r = PossibleRange::new();
        for rule in rules {
            rule.apply(&mut r);
        }
        part2 += r.size();
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(409898, part1);
        assert_eq!(113057405770956, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
