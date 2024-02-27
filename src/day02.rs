use aoc2023::ZipWith;
use project_root::get_project_root;

use nom::{bytes::complete, character, multi, IResult, branch};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Game {
    id: u32,
    max_rgb: [u32; 3],
}

// Game 1: 7 red, 8 blue; 6 blue, 6 red, 2 green; 2 red, 6 green, 8 blue; 9 green, 2 red, 4 blue; 6 blue, 4 green

fn parse_rgb(input: &str) -> IResult<&str, [u32; 3]> {
    let (input, n) = character::complete::u32(input)?;
    let (input, _) = character::complete::space1(input)?;
    let (input,name) = character::complete::alpha1(input)?;
    Ok((
        input,
        match name {
            "red" => [n, 0, 0],
            "green" => [0, n, 0],
            "blue" => [0, 0, n],
            _ => [0, 0, 0]
        },
    ))
}
fn parse_game(input: &str) -> IResult<&str, Game> {
    let max_rgb = [u32::min_value(); 3];
    let (input, _) = complete::tag("Game ")(input)?;
    let (input, id) = character::complete::u32(input)?;
    let (input, _) = complete::tag(": ")(input)?;
    let (input, rgb) = multi::separated_list1(branch::alt((complete::tag(", "), complete::tag("; "))), parse_rgb)(input)?;
    let max_rgb = rgb.into_iter().fold(max_rgb, |acc, x| {acc.zip_with(|a, b| *a.max(b), x)});
    Ok((input, Game { id, max_rgb }))
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (_, game) = multi::separated_list0(character::complete::newline, parse_game)(&input).unwrap();
    let min_game = [12, 13, 14];
    let day2a = game.iter().filter_map(|g| {
        if g.max_rgb.zip_with(|x, y| x <= y, min_game).iter().all(|&x| x) {
            Some(g.id)
        } else {None}
    }).sum::<u32>();
    let day2b = game.iter().map(|g| g.max_rgb.iter().product::<u32>()).sum::<u32>();
    println!("day2a: {}", day2a);
    println!("day2b: {}", day2b)
}
