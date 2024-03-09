use aoc2023::Direction::{self, *};
use project_root::get_project_root;
use std::i64;

fn read_input(input: &str) -> Option<(Direction, f64)> {
    let mut input = input.split_whitespace();
    Some((
        match input.next()? {
            "U" => North,
            "L" => West,
            "R" => East,
            "D" => South,
            _ => panic!("wrong char"),
        },
        input.next()?.parse::<f64>().ok()?,
    ))
}
type V = Vec<(Direction, f64)>;
type Index = (f64, f64);
fn calc_corners(d0: &Direction, d1: &Direction, (x, y): &Index) -> (Index, Index) {
    match d0 {
        South => match d1 {
            East => ((x + 0.5, y - 0.5), (x - 0.5, y + 0.5)),
            West => ((x + 0.5, y + 0.5), (x - 0.5, y - 0.5)),
            _ => panic!("wrong d"),
        },
        North => match d1 {
            East => ((x - 0.5, y - 0.5), (x + 0.5, y + 0.5)),
            West => ((x - 0.5, y + 0.5), (x + 0.5, y - 0.5)),
            _ => panic!("wrong d"),
        },
        _ => calc_corners(d1, d0, &(*x, *y)), 
    }
}
fn calc_edges(input: &V) -> (Vec<Index>, Vec<Index>) {
    let mut start = (0.0, 0.0);
    let mut input = input.iter().peekable();
    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    while let (Some((d0, _)), Some((d1, n1))) = (input.next(), input.peek()) {
        let (p, s) = calc_corners(d0, d1, &start);
        v0.push(p);
        v1.push(s);
        start = (
            start.0 + (d1.to_index().0 as f64 * n1),
            start.1 + (d1.to_index().1 as f64 * n1),
        );
    }
    (v0, v1)
}
fn calc_area(v: &Vec<Index>) -> f64 {
    let v0 = v.first().unwrap();
    let mut v = v.clone();
    v.push(*v0);
    let mut v = v.iter().peekable();
    let mut acc = 0.0f64;
    while let (Some((x0, y0)), Some((x1, y1))) = (v.next(), v.peek()) {
        acc += x0 * y1 - y0 * x1;
    }
    acc.abs() / 2.0
}
fn read_input_b(input: &str) -> Option<(Direction, f64)> {
    let (a, b) = input.split_whitespace().nth(2)?.trim_start_matches("(#").trim_end_matches(')').split_at(5);
    // dbg!(&a, &b);
    let b = (match b {
        "0" => Some(East),
        "1" => Some(South),
        "2" => Some(West),
        "3" => Some(North),
        _ => None,
    })?;
    Some((b, i64::from_str_radix(a, 16).ok()? as f64))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut ins_a = input.lines().filter_map(read_input).collect::<Vec<_>>();
    ins_a.push(*ins_a.first().unwrap());
    let (a, b) = calc_edges(&ins_a);
    println!("day18a: {}", calc_area(&a).max(calc_area(&b)));
    let mut ins_a = input.lines().filter_map(read_input_b).collect::<Vec<_>>();
    ins_a.push(*ins_a.first().unwrap());
    let (a, b) = calc_edges(&ins_a);
    println!("day18b: {}", calc_area(&a).max(calc_area(&b)));
}
