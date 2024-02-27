use std::{collections::HashMap, mem::swap};

use project_root::get_project_root;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, newline, space0, space1, u64},
    multi::{self, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Conversion {
    destination: String,
    s_to_d: Vec<(u64, u64, u64)>,
}

type ConversionMap = HashMap<String, Conversion>;
fn input_parser(input: &str) -> IResult<&str, (Vec<u64>, ConversionMap)> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(space1, u64))(input)?;
    let (input, c) = preceded(
        many1(newline),
        separated_list1(many1(newline), conversion_parser),
    )(input)?;
    let mut o = HashMap::new();
    for m in c {
        o.extend(m);
    }
    Ok((input, (seeds, o)))
}
fn conversion_parser(input: &str) -> IResult<&str, ConversionMap> {
    let (input, source) = terminated(alpha1, tag("-to-"))(input)?;
    let (input, destination) = terminated(alpha1, tag(" map:\n"))(input)?;
    let (input, mut s_to_d) = separated_list1(
        newline,
        tuple((terminated(u64, space1), terminated(u64, space1), u64)),
    )(input)?;
    s_to_d.iter_mut().for_each(|(x, y, _z)| swap(x, y));
    s_to_d.sort();
    Ok((
        input,
        HashMap::from([(
            String::from(source),
            Conversion {
                destination: String::from(destination),
                s_to_d,
            },
        )]),
    ))
}
type Range = (u64, u64);
fn convert_ragne(source: &str, dest: &str, input: Vec<Range>, m: &ConversionMap) -> Vec<Range> {
    let mut source = String::from(source);
    let mut output = input;
    let mut input = Vec::new();
    while source != dest {
        let v = m.get(&source).unwrap();
        input = std::mem::take(&mut output);
        // dbg!(&input);
        for n in input {
            let (mut i, mut j) = n;
            for (a, b, c) in &v.s_to_d {
                // dbg!((a, b, c, i, j, &output));
                match i {
                    x if x < *a && j > *a => {
                        output.push((i, *a));
                        if j > a + c {
                            output.push((*b, b + c));
                            i = a + c;
                        } else {
                            output.push((*b, j - a + b));
                            i = j;
                        }
                    }
                    x if x >= *a && x < a + c => {
                        if j > a + c {
                            output.push((i - a + b, b + c));
                            i = a + c;
                        } else {
                            output.push((i - a + b, j - a + b));
                            i = j;
                        }
                    }
                    _ => continue,
                }
            }
            if j > i {
                output.push((i, j));
            }
        }
        source = v.destination.clone();
    }
    output
}
fn convert(source: &str, dest: &str, n: u64, m: &ConversionMap) -> Option<u64> {
    let mut source = String::from(source);
    let mut n = n;
    while source != dest {
        let v = m.get(&source)?;
        for (a, b, c) in &v.s_to_d {
            match n {
                x if x < a + c && x >= *a => {
                    n = x - a + b;
                    break;
                }
                _ => continue,
            }
        }
        source = v.destination.clone();
    }
    Some(n)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (_, (seeds, m)) = input_parser(&input).unwrap();
    let day5a = seeds
        .iter()
        .filter_map(|x| convert("seed", "location", *x, &m))
        .min()
        .unwrap();
    let seeds_range = seeds
        .chunks(2)
        .map(|v| (v[0], v[0] + v[1]))
        .collect::<Vec<Range>>();
    println!("day5a: {}", day5a);
    // dbg!(convert_ragne("seed", "location", seeds_range, &m).iter().map(|(x, y)| y - x).sum::<u64>());
    println!(
        "day5b: {}",
        convert_ragne("seed", "location", seeds_range, &m)
            .iter()
            .filter_map(|(x, y)| if y > x {Some(x)} else {None})
            .min()
            .unwrap()
    );
}
