use std::mem::take;
use aoc2023::{
    build_map,
    Direction::{self, *},
    Enum,
};
use project_root::get_project_root;
use std::collections::{BTreeMap, BTreeSet};

type Pipe = char;

fn thru_pipe(pipe: &Pipe, dir_in: &Direction) -> Option<Direction> {
    match (pipe, dir_in) {
        ('-', East) => Some(East),
        ('-', West) => Some(West),
        ('|', South) => Some(South),
        ('|', North) => Some(North),
        ('L', West) => Some(North),
        ('L', South) => Some(East),
        ('J', East) => Some(North),
        ('J', South) => Some(West),
        ('7', North) => Some(West),
        ('7', East) => Some(South),
        ('F', West) => Some(South),
        ('F', North) => Some(East),
        _ => None,
    }
}
type M = BTreeMap<Index, char>;
type S = BTreeSet<Index>;
type Index = (isize, isize);

fn add_index(i: &Index, j: &Index) -> Index {
    (i.0 + j.0, i.1 + j.1)
}

fn flood(l: S, r: S, p: &S) -> S {
    let mut l = l;
    let mut r = r;
    let mut ln = BTreeSet::new();
    let mut rn = BTreeSet::new();
    let mut accl = BTreeSet::new();
    let mut accr = BTreeSet::new();
    while !l.is_empty() && !r.is_empty() {
        for i in l.iter() {
            ln
                .extend((0..=3).filter_map(|j| {
                    let d = add_index(i, &Direction::to_enum(j).to_index());
                    if accl.contains(&d) || p.contains(&d) {
                        None
                    } else {
                        Some(d)
                    }
                }));
            accl.insert(*i);
        }
        l = take(&mut ln);
        for i in r.iter() {
            rn
                .extend((0..=3).filter_map(|j| {
                    let d = add_index(i, &Direction::to_enum(j).to_index());
                    if accr.contains(&d) || p.contains(&d) {
                        None
                    } else {
                        Some(d)
                    }
                }));
            accr.insert(*i);
        }
        r = take(&mut rn);
    }
    if l.is_empty() {
        accl
    } else {
        accr
    }
}
fn walk_map(m: &M) -> (Index, S, S, S) {
    let start_point = m
        .iter()
        .find_map(|(x, &y)| if y == 'S' { Some(*x) } else { None })
        .unwrap();
    let start_dir: [Direction; 2] = (0..=3)
        .filter_map(|i| {
            let d = Direction::to_enum(i);
            let x = add_index(&start_point, &d.to_index());
            let _next_d = thru_pipe(m.get(&x)?, &d)?;
            Some(d)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut bot = (start_point, start_dir[0]);
    let mut path = BTreeSet::new();
    let mut left = BTreeSet::new();
    let mut right = BTreeSet::new();
    while !path.contains(&bot.0) {
        // dbg!(&bot);
        path.insert(bot.0);
        left.insert(add_index(&bot.0, &bot.1.pred().to_index()));
        right.insert(add_index(&bot.0, &bot.1.succ().to_index()));
        let i = add_index(&bot.0, &bot.1.to_index());
        left.insert(add_index(&i, &bot.1.pred().to_index()));
        right.insert(add_index(&i, &bot.1.succ().to_index()));
        let d = (|| {
            let d = thru_pipe(m.get(&i)?, &bot.1)?;
            Some(d)
        })()
        .unwrap_or(start_dir[1]);
        bot = (i, d);
    }
    left.retain(|x| !path.contains(x));
    right.retain(|x| !path.contains(x));
    (start_point, path, left, right)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let m = build_map(input.lines().map(|x| x.chars()), |(a, b), y| {
        if y == '.' {
            None.into_iter()
        } else {
            Some(((a as isize, b as isize), y)).into_iter()
        }
    });
    let day10 = walk_map(&m);
    println!("day10a: {}", &day10.1.len());
    println!("day10b: {}", flood(day10.2, day10.3, &day10.1).len());
}
