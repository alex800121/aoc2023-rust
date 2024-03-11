use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, alpha1, char, newline},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, terminated},
    sequence::{preceded, tuple},
    IResult,
};
use project_root::get_project_root;

type Ins<'a> = HashMap<&'a str, Vec<(Cond, Dest<'a>)>>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Cond {
    LessThan(usize, u32),
    GreaterThan(usize, u32),
    Pass,
}
#[derive(Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Debug, Hash)]
enum Dest<'a> {
    Accepted,
    Rejected,
    Field(&'a str),
}
fn xmas_parser(input: &str) -> IResult<&str, usize> {
    alt((
        value(0, char('x')),
        value(1, char('m')),
        value(2, char('a')),
        value(3, char('s')),
    ))(input)
}
fn dest_parser(input: &str) -> IResult<&str, Dest<'_>> {
    use Dest::*;
    alt((
        value(Accepted, char('A')),
        value(Rejected, char('R')),
        map(alpha1, Field),
    ))(input)
}
fn cond_parser_0(input: &str) -> IResult<&str, Cond> {
    use Cond::*;
    let (input, f) = xmas_parser(input)?;
    let (input, c) = alt::<_, _, _, _>((char('<'), char('>')))(input)?;
    let (input, i) = terminated(complete::u32, char(':'))(input)?;
    Ok((
        input,
        if c == '<' {
            LessThan(f, i)
        } else {
            GreaterThan(f, i)
        },
    ))
}
fn cond_parser(input: &str) -> IResult<&str, Cond> {
    cond_parser_0(input).or(Ok((input, Cond::Pass)))
}
type Xmas = [u32; 4];
type Range = (u32, u32);
fn ins_parser(input: &str) -> IResult<&str, (&str, Vec<(Cond, Dest<'_>)>)> {
    let (input, name) = alpha1(input)?;
    let (input, x) = delimited(
        char('{'),
        separated_list1(char(','), tuple((cond_parser, dest_parser))),
        char('}'),
    )(input)?;
    Ok((input, (name, x)))
}
fn input_parser(input: &str) -> IResult<&str, (Ins<'_>, Vec<Xmas>)> {
    let (input, m) = separated_list1(newline, ins_parser)(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    let (input, l) = separated_list1(
        newline,
        delimited(
            char('{'),
            separated_list1(
                char(','),
                preceded(tuple((xmas_parser, char('='))), complete::u32),
            ),
            char('}'),
        ),
    )(input)?;
    Ok((
        input,
        (
            HashMap::from_iter(m),
            l.into_iter().map(|x| x.try_into().unwrap()).collect(),
        ),
    ))
}
type XmasPlus = [Range; 4];
fn fill_next<'a>(cond: &Vec<(Cond, Dest<'a>)>, r: XmasPlus) -> Vec<(XmasPlus, Dest<'a>)> {
    use Cond::*;
    let mut v = Vec::new();
    let mut init = vec![r];
    let mut rest = Vec::new();
    for (c, dest) in cond {
        for r in init.drain(..) {
            match c {
                Pass => v.push((r, *dest)),
                LessThan(f, i) => match r[*f] {
                    (_, n) if n <= *i => v.push((r, *dest)),
                    (m, n) if m < *i => {
                        let mut x = r;
                        x[*f] = (m, *i);
                        v.push((x, *dest));
                        x[*f] = (*i, n);
                        rest.push(x);
                    }
                    _ => rest.push(r),
                },
                GreaterThan(f, i) => match r[*f] {
                    (m, _) if m > *i => v.push((r, *dest)),
                    (m, n) if n > *i + 1 => {
                        let mut x = r;
                        x[*f] = (*i + 1, n);
                        v.push((x, *dest));
                        x[*f] = (m, *i + 1);
                        rest.push(x);
                    }
                    _ => rest.push(r),
                },
            }
        }
        init = std::mem::take(&mut rest);
    }
    v
}
fn calc_range(range: [Range; 4], start: &str, ins: &Ins<'_>) -> (Vec<XmasPlus>, Vec<XmasPlus>) {
    use Dest::*;
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    let mut init_range = vec![(range, Field(start))];
    let mut next_range = Vec::new();
    while !init_range.is_empty() {
        for (r, s) in init_range {
            match s {
                Accepted => accepted.push(r),
                Rejected => rejected.push(r),
                Field(s) => {
                    if let Some(cond) = ins.get(s) {
                        let v = fill_next(cond, r);
                        next_range.extend(v);
                    }
                }
            }
        }
        init_range = std::mem::take(&mut next_range);
    }
    (accepted, rejected)
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (ins, v) = input_parser(&input).unwrap().1;
    let a: u64 = v
        .into_iter()
        .map(|a| {
            let a = a.map(|x| (x, x + 1));
            calc_range(a, "in", &ins)
                .0
                .into_iter()
                .map(|x| {
                    x.iter()
                        .map(|x| if x.1 > x.0 { x.0 as u64 } else { 0 })
                        .sum::<u64>()
                })
                .sum::<u64>()
        })
        .sum();
    println!("day19a: {}", a);
    println!(
        "day19b: {}",
        calc_range([(1, 4001); 4], "in", &ins)
            .0
            .iter()
            .map(|x| x.iter().map(|(a, b)| (b - a) as u64).product::<u64>())
            .sum::<u64>()
    );
}
