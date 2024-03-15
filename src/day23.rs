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
const LEN: usize = 36;
// type Vertex = (Index, Index);
type KMap = [[Option<usize>; LEN]; LEN];
fn bfs(start: &Index, end: &Index, m: &M) -> KMap {
    let mut kmap: KMap = [[None; LEN]; LEN];
    let mut joints = vec![*start, *end];
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
    let joints: [Index; LEN] = joints.try_into().unwrap();
    // dbg!(joints.len());
    // dbg!(&joints);
    for (i0, s) in joints.iter().enumerate() {
        let mut n = 0;
        let mut start0 = vec![*s];
        let mut visited = BTreeSet::new();
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
                        match (m[y as usize][x as usize], joints.iter().find_position(|&&k| k == s1)) {
                            (_, Some((i1, _))) => {
                                kmap[i0][i1] = Some(n + 1);
                                // if let Some(m) = kmap.get_mut(&s) {
                                //     m.insert(*end, n + 1);
                                // } else {
                                //     kmap.insert(s, BTreeMap::from([(*end, n + 1)]));
                                // }
                                // kmap.insert((s, *end), n + 1);
                            },
                            (c, _) if c != '#' && c != to_char(d.succ().succ()) => {
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

fn all_routes(start: usize, end: usize, kmap: &KMap) -> Vec<usize> {
    let mut start = vec![(start, 0u64, 0)];
    let mut acc = vec![];
    while !start.is_empty() {
        // dbg!(&start);
        let mut next_start = vec![];
        for (s0, visited0, n0) in start.drain(..) {
            if s0 == end {
                acc.push(n0);
            } else {
                let visited1 = visited0 | (1 << s0);
                let m = kmap[s0];
                for (s1, n1) in m.iter().enumerate().filter_map(|x| {
                    x.1.map(|y| (x.0, y))
                }) {
                    if (visited1 >> s1) & 1 == 0 {
                        next_start.push((s1, visited1, n1 + n0));
                    }
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
    let a = all_routes(0, 1, &kmap);
    println!("day23a: {}", a.iter().max().unwrap());
    let mut kmap0: KMap = [[None; LEN]; LEN];
    for (k0, l) in kmap.iter().enumerate() {
        for (k1, n) in l.iter().enumerate().filter_map(|x| x.1.map(|y| (x.0, y))) {
            kmap0[k0][k1] = Some(n);
            kmap0[k1][k0] = Some(n);
        }
    }
    let b = all_routes(0, 1, &kmap0);
    println!("day23b: {}", b.iter().max().unwrap());
}
