use aoc2023::ZipWith;
use nom::{
    character::complete::{char, i32, newline},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use project_root::get_project_root;

type Range = (i32, i32);
type Brick = [Range; 3];

fn overlap(a: Brick, b: Brick) -> bool {
    a.zip_with(|(x0, y0), (x1, y1)| y1.min(y0) > x0.max(x1), b)
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
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input_parser(&input).unwrap().1;
    dbg!(input);
}
