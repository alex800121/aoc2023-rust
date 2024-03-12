use std::{array, collections::HashSet, ops::Div};

use num::Integer;
use project_root::get_project_root;

const N: usize = 131;

type M<T> = [[T; N]; N];

type Index = (isize, isize);
const ADJACENT: [Index; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
fn bfs(m: &M<char>, mut start: HashSet<(usize, usize)>) -> Mu {
    let mut output = [[None; N]; N];
    let mut visited = HashSet::new();
    let mut next_start = HashSet::new();
    let mut n = 0;
    while !start.is_empty() {
        for i @ (x, y) in start.drain() {
            visited.insert(i);
            output[y][x] = Some(n);
            for (a, b) in ADJACENT.iter() {
                let x = x as isize + a;
                let y = y as isize + b;
                if x >= 0
                    && y >= 0
                    && x < N as isize
                    && y < N as isize
                    && m[y as usize][x as usize] != '#'
                    && !visited.contains(&(x as usize, y as usize))
                {
                    next_start.insert((x as usize, y as usize));
                }
            }
        }
        start = std::mem::take(&mut next_start);
        n += 1;
    }
    output
}
/*
###   ###   ###   ###
###   ###   ###   ###
###   ###   ###   ###
   ###   ###   ###
   ###   ###   ###
   ###   ###   ###
###   ###   ###   ###
###   ###   ###   ###
###   ###   ###   ###
   ###   ###   ###
   ###   #S#   ###
   ###   ###   ###
###   ###   ###   ###
###   ###   ###   ###
###   ###   ###   ###
   ###   ###   ###
   ###   ###   ###
   ###   ###   ###
###   ###   ###   ###
###   ###   ###   ###
###   ###   ###   ###
n = 10, d = 3
d2 = 4
(ae, be) = (10 - 4).div_mod(6) = (1, 0)
(ao, bo) = (10 - 4 - 3).div_mod(6) = (0, 3)
*/
type Mu = M<Option<usize>>;
fn calc_ceo_seo_center(n: u64, d: u64) -> [(u64, (u64, u64)); 5] {
    let d2 = (d / 2) * 2 + 2;
    let (ae, be) = (n - d2).div_mod_floor(&(d * 2));
    let (ao, bo) = (n - d2 - d).div_mod_floor(&(d * 2));
    let cec = ae.pow(2);
    let cep = (ae + 1).pow(2) - cec;
    let coc = ao * (ao + 1);
    let cop = (ao + 1) * (ao + 2) - coc;
    let (sea, seb) = (n - (d / 2) - 1).div_mod_floor(&(d * 2));
    let (soa, sob) = (n - (d / 2) - 1 - d).div_mod_floor(&(d * 2));
    [
        (cec, (cep, be)),
        (coc, (cop, bo)),
        (sea, (1, seb)),
        (soa, (1, sob)),
        (0, (1, n)),
    ]
}
fn build(m: &M<char>) -> ([Mu; 4], [Mu; 4], Mu) {
    let a @ (cx, cy) = (N / 2, N / 2);
    let max_x = N - 1;
    let max_y = N - 1;
    let c = [(0, max_y), (max_x, max_y), (0, 0), (max_x, 0)].map(|c| {
        let start = HashSet::from([c]);
        bfs(m, start)
    });
    let s = [(0, cy), (max_x, cy), (cx, 0), (cx, max_y)].map(|c| {
        let start = HashSet::from([c]);
        bfs(m, start)
    });
    let a = bfs(m, HashSet::from([a]));
    (c, s, a)
}
fn calc_sum<T>(cond: T, m: &Mu, (complete, (partial, limit)): (u64, (u64, u64))) -> u64
where
    T: Fn(u64) -> bool,
{
    let p = m
        .map(|xs| {
            xs.iter()
                .filter(|&&x| {
                    x.map(|x| cond(x as u64) && x as u64 <= limit)
                        .unwrap_or(false)
                })
                .count() as u64
        })
        .iter()
        .sum::<u64>();
    let c = m
        .map(|xs| {
            xs.iter()
                .filter(|&&x| x.map(|x| cond(x as u64)).unwrap_or(false))
                .count() as u64
        })
        .iter()
        .sum::<u64>();
    complete * c + partial * p
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: M<char> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let start = HashSet::from_iter(input.iter().enumerate().find_map(|(y, xs)| {
        xs.iter()
            .enumerate()
            .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
    }));
    println!(
        "day21a: {}",
        bfs(&input, start)
            .iter()
            .map(|x| x
                .iter()
                .filter(|&&c| c.map(|c| c <= 64 && c % 2 == 0).unwrap_or(false))
                .count() as u64)
            .sum::<u64>()
    );
    let [ce, co, se, so, center] = calc_ceo_seo_center(26501365, N as u64);
    let (cor, sid, cen) = build(&input);
    let b = {
        let mut acc = 0;
        for c in cor {
            acc += calc_sum(|x| x % 2 != 0, &c, ce);
            acc += calc_sum(|x| x % 2 == 0, &c, co);
        }
        for s in sid {
            acc += calc_sum(|x| x % 2 != 0, &s, se);
            acc += calc_sum(|x| x % 2 == 0, &s, so);
        }
        acc += calc_sum(|x| x % 2 != 0, &cen, center);
        acc
    };
    println!("day21b: {}", b);
}
