use std::collections::{BTreeMap, BTreeSet};

use project_root::get_project_root;

use nom::{branch, character, IResult};

type Index = (i32, i32);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Part {
    Symbol(char, Index),
    PartNum(u32, Index, i32),
}

fn num_parser<'a>((x, y): &Index, input: &'a str) -> IResult<&'a str, (Index, Option<Part>)> {
    let (input, a) = character::complete::digit1(input)?;
    let l = a.len() as i32;
    let n = a.parse::<u32>().unwrap();
    Ok((input, ((x + l, *y), Some(Part::PartNum(n, (*x, *y), l)))))
}

fn symbol_parser<'a>((x, y): &Index, input: &'a str) -> IResult<&'a str, (Index, Option<Part>)> {
    // fn symbol_parser((x, y): Index, input: &str) -> IResult<&str, (Index, Option<Part>)> {
    let (input, c) =
        nom::character::complete::satisfy(|x| x != '.' && x != '\n' && !x.is_ascii_digit())(input)?;
    Ok((input, ((x + 1, *y), Some(Part::Symbol(c, (*x, *y))))))
}

fn newline_parser<'a>((_x, y): &Index, input: &'a str) -> IResult<&'a str, (Index, Option<Part>)> {
    // fn newline_parser((x, y): Index, input: &str) -> IResult<&str, (Index, Option<Part>)> {
    let (input, _) = character::complete::newline(input)?;
    Ok((input, ((0, y + 1), None)))
}
fn space_parser<'a>((x, y): &Index, input: &'a str) -> IResult<&'a str, (Index, Option<Part>)> {
    // fn space_parser((x, y): Index, input: &str) -> IResult<&str, (Index, Option<Part>)> {
    let (input, _) = character::complete::char('.')(input)?;
    Ok((input, ((x + 1, *y), None)))
}

fn input_parser(input: &str) -> IResult<&str, Vec<Part>> {
    let mut ix = (0, 0);
    let mut inputx = input;
    let mut acc = Vec::new();
    let mut i0;
    loop {
        i0 = ix;
        if let Ok((inputy, (i, p))) = branch::alt((
            |x| newline_parser(&i0, x),
            |x| space_parser(&i0, x),
            |x| num_parser(&i0, x),
            |x| symbol_parser(&i0, x),
        ))(inputx)
        {
            inputx = inputy;
            ix = i;
            if let Some(p) = p {
                acc.push(p);
            }
        } else {
            break;
        }
    }
    Ok((inputx, acc))
}

fn split_parts(parts: Vec<Part>) -> (Vec<(Index, char)>, Vec<(BTreeSet<Index>, u32)>) {
    let mut p = Vec::new();
    let mut n = Vec::new();
    for v in parts {
        match v {
            Part::PartNum(x, (a, b), z) => {
                let l = (0..z).map(|q| (a + q, b)).collect();
                n.push((l, x));
            }
            Part::Symbol(c, i) => {
                p.push((i, c));
            }
        }
    }
    (p, n)
}
fn surrounds((x0, y0): &Index, (x1, y1): &Index) -> bool {
    let v = [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)];
    v.iter().any(|(x, y)| *x0 == x + x1 && *y0 == y + y1)
}
fn group_parts(parts: Vec<Part>) -> BTreeMap<(Index, char), Vec<u32>> {
    let (p, mut n) = split_parts(parts);
    let mut m: BTreeMap<(Index, char), Vec<u32>> = BTreeMap::new();
    for (i, c) in p {
        n = n.into_iter().filter_map(|(x, y)| {
            if x.iter().any(|a| surrounds(a, &i)) {
                if let Some(v) = m.get_mut(&(i, c)) {
                    v.push(y);
                } else {
                    m.insert((i, c), vec![y]);
                }
                None
            } else {
                Some((x, y))
            }
        }).collect();
    }
    m
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (_, v) = input_parser(&input).unwrap();
    let grouped = group_parts(v);
    let day3a = grouped.values().map(|v| v.iter().sum::<u32>()).sum::<u32>();
    let day3b = grouped.iter().filter_map(|((_i, k), x)| {
        if *k == '*' && x.len() == 2 {
            Some(x.iter().product::<u32>())
        } else {
            None
        }
    }).sum::<u32>();
    println!("day3a: {}", day3a);
    println!("day3b: {}", day3b);
}
