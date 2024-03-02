use project_root::get_project_root;
use std::mem::take;

fn calc_end(input: Vec<isize>) -> (isize, isize) {
    let mut v = input;
    let mut acc = Vec::new();
    while v.iter().any(|x| x != &0) {
        let mut c = v.iter();
        let mut x = c.next().unwrap_or(&0);
        let mut o = Vec::new();
        for i in c {
            o.push(i - x);
            x = i;
        }
        acc.push(v);
        v = take(&mut o);
    }
    let mut h = 0;
    let mut e = 0;
    while let Some(x) = acc.pop() {
        e += x.iter().last().unwrap();
        h = x.first().unwrap() - h;
    }
    (h, e)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|x| x.parse::<isize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let a = input
        .into_iter()
        .map(calc_end)
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("day9a: {}", a.1);
    println!("day9b: {}", a.0);
}
