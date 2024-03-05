use aoc2023::Direction::{self, *};
use project_root::get_project_root;

type Bot = (Index, Direction);
type Index = (usize, usize);
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: [[char; 110]; 110] = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    dbg!(input);
}
