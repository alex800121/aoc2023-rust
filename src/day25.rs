use itertools::Itertools;
use project_root::get_project_root;
use std::collections::HashMap;
use petgraph::{graph::UnGraph, visit::{EdgeCount, EdgeRef}};
use rand::{prelude::*, rngs::SmallRng};

fn to_u8(s: &str) -> Option<[u8; 3]> {
    s.bytes().collect_vec().try_into().ok()
}
type HMap = HashMap<[u8; 3], u32>;
type G = UnGraph<u32, u32>;
fn get_hmap(hmap: &mut HMap, max_index: &mut u32, s: [u8; 3]) -> u32 {
    if let Some(x) = hmap.get(&s) {
        *x
    } else {
        hmap.insert(s, *max_index);
        *max_index += 1;
        *max_index - 1
    }
}
fn karger(g: &mut G, s: u64) {
    let mut rng = SmallRng::seed_from_u64(s);
    loop {
        if g.edge_count() <= 1 {
            return;
        }
        let m = g.edge_weights().max().unwrap();
        let max_edges = g.edge_references().filter(|x| x.weight() == m).collect_vec();
        let selected = max_edges.get(rng.next_u32() as usize % max_edges.len()).unwrap().id();
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let g = {
        let mut max_index = 0;
        let mut hmap: HMap = HashMap::new();
        let mut xy = Vec::new();
        for s in input.lines() {
            (|| {
                let (x, ys) = s.split_once(": ")?;
                let mut t = Vec::new();
                for y in ys.split_whitespace().filter_map(to_u8) {
                    t.push(get_hmap(&mut hmap, &mut max_index, y));
                }
                xy.push((get_hmap(&mut hmap, &mut max_index, to_u8(x)?), t));
                Some(())
            })();
        }
        // let u = vec![0; max_index];
        let mut v = Vec::new();
        // v.resize(max_index, u.clone());
        for (x, ys) in xy {
            for y in ys {
                // (|| {
                //     let a = v.get_mut(x)?;
                //     let b = a.get_mut(y)?;
                //     *b = 1;
                //     let a = v.get_mut(y)?;
                //     let b = a.get_mut(x)?;
                //     *b = 1;
                //     Some(())
                // })();
                v.push((x, y, 1));
            }
        }
        UnGraph::<u32, u32>::from_edges(v)
    };
    // dbg!(g.iter().map(|v| v.iter().sum::<usize>()).sum::<usize>());
    dbg!(g);
}
