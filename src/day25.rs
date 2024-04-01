use itertools::Itertools;
use petgraph::{
    data::{Build, DataMap},
    graph::{Node, UnGraph},
    visit::{EdgeRef, IntoEdgeReferences},
};
use project_root::get_project_root;
use rand::{prelude::*, rngs::SmallRng};
use std::collections::HashMap;

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
// fn karger(g: &G) -> G {
fn karger(g: &G) -> u32 {
    let mut s = 0;
    's: loop  {
        let mut g = g.clone();
        let mut rng = SmallRng::seed_from_u64(s);
        loop {
            // dbg!(&g);
            if g.edge_count() <= 1 {
                let e = g.edge_references().find_or_first(|_| true).unwrap();
                if *e.weight() == 3 {
                    return g.node_weight(e.source()).unwrap() * g.node_weight(e.target()).unwrap();
                    // return g;
                } else {
                    s += 1;
                    continue 's;
                }
            }
            let m = g.edge_weights().max().unwrap();
            let max_edges = g
                .edge_references()
                .filter(|x| x.weight() == m)
                .collect_vec();
            let selected = max_edges
                .get(rng.next_u32() as usize % max_edges.len())
                .unwrap();
            let source = selected.source();
            let target = selected.target();
            let node_weight = *g.node_weight(target).unwrap();
            // dbg!(&selected, &source, &target);
            for n in g.neighbors_undirected(target).collect_vec() {
                if n != source {
                    let w = *g
                        .edge_weight(g.find_edge_undirected(target, n).unwrap().0)
                        .unwrap();
                    // dbg!(&w);
                    if let Some(e) = g
                        .find_edge_undirected(source, n)
                        .or(g.find_edge_undirected(n, source))
                        .and_then(|x| g.edge_weight_mut(x.0))
                    {
                        *e += w;
                    } else {
                        g.add_edge(source, n, w);
                    }
                }
            }
            if let Some(w) = g.node_weight_mut(source) {
                *w += node_weight;
            }
            g.remove_node(target);
        }
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
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
                v.push((x, y, 1));
            }
        }
        let mut v = UnGraph::<u32, u32>::from_edges(v);
        v.node_weights_mut().for_each(|n| *n = 1);
        v
    };
    // dbg!(g.iter().map(|v| v.iter().sum::<usize>()).sum::<usize>());
    println!("day25a: {}", karger(&g));
    println!("Merry Christmas!");
}
