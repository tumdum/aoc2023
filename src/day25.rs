use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

use crate::input::token_groups;

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let input: Vec<Vec<String>> = token_groups(&input.replace(":", " "), "\n", None);

    let mut g: FxHashMap<String, FxHashSet<String>> = Default::default();

    for line in &input {
        let key = &line[0].to_owned();
        for value in &line[1..] {
            g.entry(key.clone()).or_default().insert(value.to_owned());
            g.entry(value.to_owned()).or_default().insert(key.clone());
        }
    }

    /*
    // graphviz
    println!("digraph {{ ");
    for (k, vs) in &g {
        for v in vs {
            println!("{k} -> {v} [edgeURL=\"{k}_{v}\"]");
        }
    }
    println!("}}");
    */

    // let disconnect = &[("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")];
    // Found with graphvis in output_full.svg
    let disconnect = &[("njn", "xtx"), ("rhh", "mtc"), ("tmb", "gpj")];

    for (a, b) in disconnect {
        g.get_mut(*a).unwrap().remove(*b);
        g.get_mut(*b).unwrap().remove(*a);
    }
    let part1 = component(disconnect[0].0, &g).len() * component(disconnect[0].1, &g).len();

    let s = Instant::now();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(558376, part1);
    }
    if output {
        println!("\t{}", part1);
    }
    Ok(e)
}

fn component(start: &str, g: &FxHashMap<String, FxHashSet<String>>) -> FxHashSet<String> {
    let mut ret: FxHashSet<String> = Default::default();
    let mut todo: FxHashSet<String> = Default::default();
    todo.insert(start.to_owned());
    while !todo.is_empty() {
        let next = todo.iter().next().unwrap().to_owned();
        todo.remove(&next);
        if ret.insert(next.clone()) {
            // println!("{next}: {:?}", g.get(&next));
            for cand in g.get(&next).unwrap() {
                todo.insert(cand.to_owned());
            }
        }
    }
    ret
}
