use aoc2023::{
    Direction::{self, *},
    Enum,
};
use itertools::Itertools;
use project_root::get_project_root;
use std::collections::{BTreeMap, BTreeSet};
const N: usize = 141;
type M = [[char; N]; N];
type Index = (isize, isize);
// type Vertex = (Index, Index);
type KMap = BTreeMap<Index, BTreeMap<Index, usize>>;
fn bfs(start: &Index, end: &Index, m: &M) -> KMap {
    let mut kmap: KMap = BTreeMap::new();
    let mut joints = vec![*start];
    for x in 1..(N - 1) {
        for y in 1..(N - 1) {
            if [North, East, South, West]
                .iter()
                .filter(|d| {
                    let (x0, y0) = d.to_index();
                    ['^', '>', 'v', '<'].iter().any(|c| {
                        m[y][x] == '.'
                            && *c == m[(y as isize + y0) as usize][(x as isize + x0) as usize]
                    })
                })
                .count()
                > 1
            {
                joints.push((x as isize, y as isize));
            }
        }
    }
    // dbg!(&joints);
    for &s in joints.iter() {
        let mut n = 0;
        let mut start0 = vec![s];
        let mut visited = BTreeSet::new();
        // dbg!(&start0);
        while !start0.is_empty() {
            let mut next_start0 = vec![];
            for s0 @ (x0, y0) in start0.drain(..) {
                visited.insert(s0);
                for d in [North, East, South, West] {
                    let (x1, y1) = d.to_index();
                    let s1 @ (x, y) = (x0 + x1, y0 + y1);
                    if x >= 0
                        && y >= 0
                        && x < N as isize
                        && y < N as isize
                        && !visited.contains(&s1)
                    {
                        match m[y as usize][x as usize] {
                            _ if s1 == *end => {
                                if let Some(m) = kmap.get_mut(&s) {
                                    m.insert(*end, n + 1);
                                } else {
                                    kmap.insert(s, BTreeMap::from([(*end, n + 1)]));
                                }
                                // kmap.insert((s, *end), n + 1);
                            }
                            _ if joints.contains(&s1) => {
                                if let Some(m) = kmap.get_mut(&s) {
                                    m.insert(s1, n + 1);
                                } else {
                                    kmap.insert(s, BTreeMap::from([(s1, n + 1)]));
                                }
                                // kmap.insert((s, s1), n + 1);
                            }
                            c if c != '#' && c != to_char(d.succ().succ()) => {
                                next_start0.push((x, y));
                            }
                            _ => {}
                        }
                    }
                }
                // if s == (125, 123) {dbg!(&next_start0);}
            }
            start0 = next_start0;
            n += 1;
        }
    }
    kmap
}
fn to_char(d: Direction) -> char {
    match d {
        North => '^',
        East => '>',
        South => 'v',
        West => '<',
    }
}

fn all_routes_2(start: &Index, end: &Index, kmap0: &KMap) -> Vec<usize> {
    let mut kmap: KMap = BTreeMap::new();
    for (k0, l) in kmap0 {
        for (k1, n) in l {
            if let Some(x) = kmap.get_mut(k0) {
                x.insert(*k1, *n);
            } else {
                kmap.insert(*k0, BTreeMap::from([(*k1, *n)]));
            }
            if let Some(x) = kmap.get_mut(k1) {
                x.insert(*k0, *n);
            } else {
                kmap.insert(*k1, BTreeMap::from([(*k0, *n)]));
            }
        }
    }
    // dbg!(&kmap);
    let mut start = vec![(*start, BTreeSet::new(), 0)];
    let mut acc = vec![];
    while !start.is_empty() {
        // dbg!(&start);
        let mut next_start = vec![];
        for (s, mut visited, n0) in start.drain(..) {
            visited.insert(s);
            if s == *end {
                acc.push(n0);
            } else if let Some(m) = kmap.get(&s) {
                for (x, y) in m {
                    if !visited.contains(x) {
                        next_start.push((*x, visited.clone(), n0 + *y));
                    }
                }
            }
        }
        start = next_start;
    }
    acc
}
fn all_routes(start: &Index, end: &Index, kmap: &KMap) -> Vec<usize> {
    let mut start = vec![(*start, 0)];
    let mut acc = vec![];
    while !start.is_empty() {
        // dbg!(&start);
        let mut next_start = vec![];
        for (s, n0) in start.drain(..) {
            // dbg!(n0);
            if s == *end {
                acc.push(n0);
            } else if let Some(m) = kmap.get(&s) {
                for (x, y) in m {
                    next_start.push((*x, n0 + *y));
                }
            }
        }
        start = next_start;
    }
    acc
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let m: M = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let start: Index = m
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(x, n)| {
            if *n == '.' {
                Some((x as isize, 0))
            } else {
                None
            }
        })
        .unwrap();
    let end: Index = m
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(x, n)| {
            if *n == '.' {
                Some((x as isize, N as isize - 1))
            } else {
                None
            }
        })
        .unwrap();
    let kmap = bfs(&start, &end, &m);
    // dbg!(kmap);
    // dbg!(kmap.iter().find(|((a, b), n)| *a == (125, 123)));
    let a = all_routes(&start, &end, &kmap);
    println!("day23a: {}", a.iter().max().unwrap());
    let b = all_routes_2(&start, &end, &kmap);
    println!("day23b: {}", b.iter().max().unwrap());
}
