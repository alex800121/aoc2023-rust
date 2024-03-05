use std::collections::HashMap;

use aoc2023::Clockwise;
use num::Integer;
use project_root::get_project_root;

fn tilt_east<const N: usize>(v: &mut [char; N]) {
    let s = v
        .split_inclusive(|&x| x == '#')
        .map(|y| {
            y.iter().fold((0, 0, 0), |(a, b, c), &x| match x {
                '.' => (a + 1, b, c),
                'O' => (a, b + 1, c),
                '#' => (a, b, c + 1),
                _ => panic!("wrong char"),
            })
        })
        .collect::<Vec<_>>();
    let mut i = 0;
    for (a, b, c) in s.into_iter() {
        for x in 0..a {
            v[i + x] = '.';
        }
        for x in a..(a + b) {
            v[i + x] = 'O';
        }
        for x in (a + b)..(a + b + c) {
            v[i + x] = '#';
        }
        i += a + b + c;
    }
}
fn spin<const N: usize, const M: usize>(input: &mut [[char; N]; M]) {
    for _ in 0..2 {
        let mut input0 = input.clockwise();
        input0.iter_mut().for_each(tilt_east);
        *input = input0.clockwise();
        input.iter_mut().for_each(tilt_east);
    }
}
type C = [[char; 100]; 100];
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut input: C = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut input0 = input;
    input0 = input0.clockwise();
    input0.iter_mut().for_each(tilt_east);
    input0 = input0.counter_clockwise();
    println!(
        "day14a: {}",
        input0
            .iter()
            .enumerate()
            .map(|(i, r)| r.iter().filter(|&&x| x == 'O').count() * (100 - i))
            .sum::<usize>()
    );
    let mut acc:HashMap<C, usize> = HashMap::new();
    let mut i: usize = 0;
    let x = loop {
        if let Some(x) = acc.get(&input) {
            break (x, i - x);
        }
        acc.insert(input, i);
        spin(&mut input);
        i += 1;
    };
    let y = (1000000000 - x.0).mod_floor(&x.1);
    for _ in 0..y {
        spin(&mut input);
    }
    println!(
        "day14b: {}",
        input
            .iter()
            .enumerate()
            .map(|(i, r)| r.iter().filter(|&&x| x == 'O').count() * (100 - i))
            .sum::<usize>()
    );
}
