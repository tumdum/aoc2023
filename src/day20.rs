use anyhow::Result;
use rustc_hash::FxHashMap;
use smol_str::SmolStr;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::input::token_groups;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop {
        on: bool,
        outputs: Vec<SmolStr>,
    },
    Conjunction {
        last_input_pulse: FxHashMap<SmolStr, Pulse>,
        outputs: Vec<SmolStr>,
    },
    Broadcast {
        outputs: Vec<SmolStr>,
    },
    Output {
        low_received: bool,
    },
}

impl Module {
    fn add_input_name(&mut self, name: &str) {
        if let Self::Conjunction {
            last_input_pulse, ..
        } = self
        {
            last_input_pulse.insert(name.into(), Pulse::Low);
        }
    }

    fn outputs(&self) -> &[SmolStr] {
        match self {
            Module::FlipFlop { outputs, .. } => &outputs,
            Module::Conjunction { outputs, .. } => &outputs,
            Module::Broadcast { outputs } => &outputs,
            Module::Output { .. } => &[],
        }
    }

    fn process(&mut self, from: &str, pulse: Pulse) -> Vec<(SmolStr, Pulse)> {
        let mut ret = vec![];
        match self {
            Module::FlipFlop { on, outputs } => {
                if pulse == Pulse::Low {
                    let pulse = if *on { Pulse::Low } else { Pulse::High };
                    for output in outputs {
                        ret.push((output.to_owned(), pulse));
                    }
                    *on = !*on;
                }
            }
            Module::Conjunction {
                last_input_pulse,
                outputs,
            } => {
                *last_input_pulse.get_mut(from).unwrap() = pulse;
                let pulse = if last_input_pulse.values().all(|v| *v == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                for output in outputs {
                    ret.push((output.to_owned(), pulse));
                }
            }
            Module::Broadcast { outputs } => {
                for output in outputs {
                    ret.push((output.to_owned(), pulse));
                }
            }
            Module::Output { low_received } => {
                if pulse == Pulse::Low {
                    *low_received = true;
                }
            }
        }
        ret
    }
}

type Topology = FxHashMap<SmolStr, Vec<SmolStr>>;
type Kinds = FxHashMap<SmolStr, Module>;

fn send_pulse(kinds: &mut Kinds) -> (i64, i64) {
    let mut low = 0i64;
    let mut high = 0i64;
    let mut todo: VecDeque<(SmolStr, Pulse, SmolStr)> = VecDeque::default();
    todo.push_back(("broadcaster".into(), Pulse::Low, "button".into()));

    while let Some((module_name, pulse, from)) = todo.pop_front() {
        match pulse {
            Pulse::High => high += 1,
            Pulse::Low => low += 1,
        }
        let next_signals = kinds.get_mut(&module_name).unwrap().process(&from, pulse);
        for next in next_signals {
            todo.push_back((next.0, next.1, module_name.clone()));
        }
    }
    (low, high)
}

fn turned_on(kinds: &Kinds, names: &[&str]) -> usize {
    let mut ret = 0;
    for name in names {
        match kinds.get(*name) {
            Some(Module::FlipFlop { on, .. }) => {
                if *on {
                    ret += 1;
                }
            }
            _ => {}
        }
    }
    ret
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input = input.replace(",", " ");
    let input: Vec<Vec<SmolStr>> = token_groups(&input, "\n", None);

    let mut topology = Topology::default();
    let mut kinds = Kinds::default();
    let mut all = vec![];

    for line in input {
        let name = line[0].clone();

        let outputs: Vec<SmolStr> = line
            .iter()
            .skip_while(|s| *s != "->")
            .skip(1)
            .cloned()
            .collect();
        if name == "broadcaster" {
            all.push(name.clone());
            kinds.insert(name.clone(), Module::Broadcast { outputs });
        } else if name.starts_with('%') {
            let name: SmolStr = name.chars().skip(1).collect();
            all.push(name.clone());
            kinds.insert(name, Module::FlipFlop { on: false, outputs });
        } else {
            assert!(name.starts_with('&'));
            let name: SmolStr = name.chars().skip(1).collect();
            all.push(name.clone());
            kinds.insert(
                name,
                Module::Conjunction {
                    last_input_pulse: FxHashMap::default(),
                    outputs,
                },
            );
        }
    }

    for (name, module) in &kinds {
        for output in module.outputs() {
            topology
                .entry(name.to_owned())
                .or_default()
                .push(output.to_owned());
        }
    }

    let mut all2: Vec<SmolStr> = vec![];
    let mut todo: VecDeque<SmolStr> = VecDeque::new();
    todo.push_back("broadcaster".into());
    while let Some(next) = todo.pop_front() {
        if all2.contains(&next) {
            continue;
        }
        all2.push(next.clone());
        if kinds.contains_key(&next) {
            for n in kinds.get(&next).unwrap().outputs() {
                todo.push_back(n.clone());
            }
        }
    }

    for (name, outputs) in topology {
        for output in outputs {
            kinds
                .entry(output)
                .or_insert(Module::Output {
                    low_received: false,
                })
                .add_input_name(&name);
        }
    }

    let s = Instant::now();

    let part1 = {
        let mut kinds = kinds.clone();
        let mut low = 0i64;
        let mut high = 0i64;

        for _ in 0..1000 {
            let (l, h) = send_pulse(&mut kinds);
            low += l;
            high += h;
        }

        low * high
    };

    // Thank you graphiz!
    let g1 = [
        "bv", "ct", "fk", "qc", "dj", "ts", "bs", "vg", "tc", "jz", "jb", "bf",
    ];
    let g2 = [
        "hv", "kr", "rg", "zn", "mm", "ms", "zl", "hh", "np", "xb", "ds", "kz",
    ];
    let g3 = [
        "kc", "kf", "dh", "bm", "vf", "zk", "rp", "gq", "dp", "cc", "jk", "vh",
    ];
    let g4 = [
        "fm", "dx", "tx", "cm", "hl", "gr", "ns", "db", "zz", "px", "sn", "jd",
    ];

    let mut kinds = kinds.clone();
    let mut todo = vec![g1, g2, g3, g4];
    let mut part2 = 1;
    for i in 1i64.. {
        if todo.is_empty() {
            break;
        }
        send_pulse(&mut kinds);

        for idx in 0..todo.len() {
            if turned_on(&kinds, &todo[idx]) == 0 {
                if i > 1 {
                    part2 *= i;
                    todo.remove(idx);
                    break;
                }
            }
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(703315117, part1);
        assert_eq!(230402300925361, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
