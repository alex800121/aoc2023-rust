use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use num::Integer;
use project_root::get_project_root;

use std::collections::BTreeMap;
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Copy)]
enum Pulse {
    Low,
    High,
}

type Modules<'a> = BTreeMap<&'a str, Module<'a>>;
type Module<'a> = (ModuleType<'a>, Vec<&'a str>);
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { on: bool },
    Conjunction { memory: BTreeMap<&'a str, Pulse> },
}

fn broadcaster_parser(input: &str) -> IResult<&str, (&str, Module<'_>)> {
    use ModuleType::Broadcaster;
    let (input, v) = preceded(tag("broadcaster -> "), separated_list1(tag(", "), alpha1))(input)?;
    Ok((input, ("broadcaster", (Broadcaster, v))))
}
fn conjunction_parser(input: &str) -> IResult<&str, (&str, Module<'_>)> {
    use ModuleType::Conjunction;
    let (input, (n, v)) = preceded(
        char('&'),
        separated_pair(alpha1, tag(" -> "), separated_list1(tag(", "), alpha1)),
    )(input)?;
    Ok((
        input,
        (
            n,
            (
                Conjunction {
                    memory: BTreeMap::new(),
                },
                v,
            ),
        ),
    ))
}
fn flipflop_parser(input: &str) -> IResult<&str, (&str, Module<'_>)> {
    use ModuleType::FlipFlop;
    let (input, (n, v)) = preceded(
        char('%'),
        separated_pair(alpha1, tag(" -> "), separated_list1(tag(", "), alpha1)),
    )(input)?;
    Ok((input, (n, (FlipFlop { on: false }, v))))
}
fn input_parser(input: &str) -> IResult<&str, Modules<'_>> {
    use ModuleType::Conjunction;
    let (input, v) = separated_list1(
        newline,
        alt((flipflop_parser, conjunction_parser, broadcaster_parser)),
    )(input)?;
    let conjunctions = v
        .iter()
        .filter_map(|(x, (y, _))| match y {
            ModuleType::Conjunction { .. } => Some(*x),
            _ => None,
        })
        .collect::<Vec<_>>();
    let conjunctions = conjunctions
        .into_iter()
        .map(|x| {
            let y = v
                .iter()
                .filter_map(|(n, (_, v))| {
                    if v.iter().any(|y| *y == x) {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            (x, y)
        })
        .collect::<Vec<_>>();
    let mut v = BTreeMap::from_iter(v);
    for (x, y) in conjunctions {
        if let Some((Conjunction { memory }, _)) = v.get_mut(x) {
            *memory = BTreeMap::from_iter(y.into_iter().map(|n| (n, Pulse::Low)));
        }
    }
    Ok((input, v))
}
fn push_button(modules: &mut Modules<'_>) -> (u64, u64, [bool; 4]) {
    use ModuleType::*;
    use Pulse::*;
    let mut signals = vec![("button", "broadcaster", Low)];
    let mut x = 0;
    let mut y = 0;
    let r = ["js", "zb", "bs", "rr"];
    let mut t = [false; 4];
    while !signals.is_empty() {
        let mut next_signals = vec![];
        for (start, dest, sig) in signals.drain(..) {
            match sig {
                Low => x += 1,
                High => y += 1,
            };
            if let Some(i) = r.iter().enumerate().find_map(|(x, y)| {
                if *y == start && sig == High {
                    Some(x)
                } else {
                    None
                }
            }) {
                t[i] = true;
            }
            match modules.get_mut(dest) {
                Some((Broadcaster, v)) => v.iter().for_each(|&n| next_signals.push((dest, n, sig))),
                Some((FlipFlop { on }, v)) => {
                    if sig == Low {
                        let new_sig = if *on { Low } else { High };
                        v.iter()
                            .for_each(|&n| next_signals.push((dest, n, new_sig)));
                        *on = !*on;
                    }
                }
                Some((Conjunction { memory }, v)) => {
                    if let Some(p) = memory.get_mut(start) {
                        *p = sig;
                    }
                    let new_sig = if memory.iter().all(|(_, x)| *x == High) {
                        Low
                    } else {
                        High
                    };
                    v.iter()
                        .for_each(|&n| next_signals.push((dest, n, new_sig)));
                }
                None => {}
            }
        }
        signals = next_signals;
    }
    (x, y, t)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut input = input_parser(&input).unwrap().1;
    let mut input1 = input.clone();
    let (mut a, mut b) = (0, 0);
    for _ in 0..1000 {
        let (n, m, _) = push_button(&mut input);
        a += n;
        b += m;
    }
    println!("day20a: {}", a * b);
    let mut v = [vec![], vec![], vec![], vec![]];
    let mut i = 1u64;
    loop {
        if !v.iter().any(|x| x.len() <= 2) {
            break;
        }
        let l = push_button(&mut input1).2;
        for (n, b) in l.iter().enumerate() {
            if *b {
                v[n].push(i);
            }
        }
        i += 1;
    }
    assert!(v.iter().all(|x| {
        if let (Some(a), Some(b), Some(c)) = (x.first(), x.get(1), x.get(2)) {
            c - b == b - a
        } else {
            false
        }
    }));
    let mut n = 1;
    v.iter().for_each(|x| n = x.first().unwrap().lcm(&n));
    println!("day20b: {}", n);
}
