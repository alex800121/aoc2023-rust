use aoc2023::ZipWith;
use nom::{
    character::complete::{char, i32, newline},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use num::CheckedSub;
use project_root::get_project_root;
use std::{i32, collections::{BTreeMap, BTreeSet}};

type Range = (i32, i32);
type Brick = [Range; 3];
const FLOOR: Brick = [(i32::MIN, i32::MAX), (i32::MIN, i32::MAX), (0, 1)];

fn overlap(a: &Brick, b: &Brick) -> bool {
    a.zip_with(|(x0, y0), (x1, y1)| y1.min(y0) > x0.max(x1), *b)
        .iter()
        .all(|x| *x)
}
fn cord_parser(input: &str) -> IResult<&str, (i32, i32, i32)> {
    tuple((terminated(i32, char(',')), terminated(i32, char(',')), i32))(input)
}
fn brick_parser(input: &str) -> IResult<&str, Brick> {
    let (input, (a, b)) = separated_pair(cord_parser, char('~'), cord_parser)(input)?;
    let output = [(a.0, b.0 + 1), (a.1, b.1 + 1), (a.2, b.2 + 1)];
    assert!(output.iter().all(|(x, y)| y > x));
    Ok((input, output))
}
fn input_parser(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(newline, brick_parser)(input)
}
fn settle_brick(bricks: &mut Vec<Brick>) -> (SMap, SMap) {
    let mut not_settled = std::mem::take(bricks);
    let mut level = 1;
    let mut supported:SMap = BTreeMap::new();
    let mut foundation:SMap = BTreeMap::new();
    while !not_settled.is_empty() {
        let (mut lowest, rest) = not_settled.drain(..).partition(|x| x[2].0 <= level);
        not_settled = rest;
        for mut b in lowest.drain(..) {
            'a: loop {
                if overlap(&FLOOR, &b) {
                    b[2].0 += 1;
                    b[2].1 += 1;
                    if let Some(x) = foundation.get_mut(&FLOOR) {
                        x.insert(b);
                    } else {
                        foundation.insert(FLOOR, BTreeSet::from([b]));
                    }
                    if let Some(x) = supported.get_mut(&b) {
                        x.insert(FLOOR);
                    } else {
                        supported.insert(b, BTreeSet::from([FLOOR]));
                    }
                    bricks.push(b);
                    break 'a;
                }
                let mut x = bricks.iter().filter(|&x| overlap(x, &b)).collect::<Vec<_>>();
                if !x.is_empty() {
                    b[2].0 += 1;
                    b[2].1 += 1;
                    for a in x.drain(..) {
                        if let Some(x) = foundation.get_mut(a) {
                            x.insert(b);
                        } else {
                            foundation.insert(*a, BTreeSet::from([b]));
                        }
                        if let Some(x) = supported.get_mut(&b) {
                            x.insert(*a);
                        } else {
                            supported.insert(b, BTreeSet::from([*a]));
                        }
                    }
                    bricks.push(b);
                    break 'a;
                }
                b[2].0 -= 1;
                b[2].1 -= 1;
            }
        }
        level += 1
    }
    (foundation, supported)
}
type SMap = BTreeMap<Brick, BTreeSet<Brick>>;
fn calc_collapse(foundation: &SMap, supported: &SMap, brick: &Brick) -> Option<usize> {
    let mut removed = BTreeSet::from([*brick]);
    let mut n = 0;
    let mut foundation = foundation.clone();
    let mut supported = supported.clone();
    let mut acc = BTreeSet::new();
    while !removed.is_empty() {
        // dbg!(&removed);
        let mut next_removed = BTreeSet::new();
        for r in removed.into_iter() {
            if let Some(ss) = foundation.remove(&r) {
                for s in ss {
                    if let Some(a) = supported.get_mut(&s) {
                        a.remove(&r);
                    }
                }
                for (k, s) in supported.iter() {
                    if s.is_empty() {
                        next_removed.insert(*k);
                    }
                }
            }
            acc.insert(r);
        }
        removed = next_removed;
    }
    acc.len().checked_sub(1)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut input = input_parser(&input).unwrap().1;
    let (foundation, supported) = settle_brick(&mut input);
    let a = input.iter().map(|x| calc_collapse(&foundation, &supported, x)).collect::<Vec<_>>();
    dbg!(a.filter(|x| x.));
}
