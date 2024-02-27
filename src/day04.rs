use std::collections::BTreeSet;

use nom::{
    bytes::complete,
    character::complete::{char, space1, u32, newline, space0},
    multi::{self, separated_list1}, sequence::{tuple, separated_pair, preceded}, IResult,
};
use project_root::get_project_root;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct ScratchCard {
    id: u32,
    winning: BTreeSet<u32>,
    numbers: BTreeSet<u32>,
}

fn scratch_card_parser(input: &str) -> IResult<&str, ScratchCard> {
    let (input, id) = preceded(
        tuple((|x| complete::tag("Card")(x), space0)),
        u32,
    )(input)?;
    let (input, _) =
        tuple((char(':'), space1))(input)?;
    let (input, (winning, numbers)) = separated_pair(
        separated_list1(space1, u32),
        tuple((space1, char('|'), space1)),
        separated_list1(space1, u32),
    )(input)?;
    Ok((
        input,
        ScratchCard {
            id,
            winning: BTreeSet::from_iter(winning),
            numbers: BTreeSet::from_iter(numbers)
        },
    ))
}

fn calc_winning(s: &ScratchCard) -> u32 {
    let n = s.winning.intersection(&s.numbers).count();
    if n == 0 {
        0
    } else {
        2_u32.pow(n as u32 - 1)
    }
}
fn calc_cards(s: &Vec<ScratchCard>) -> u32 {
    let mut v = Vec::new();
    v.resize_with(s.len(), || 1);
    for (i, c) in s.iter().enumerate() {
        let n = c.winning.intersection(&c.numbers).count();
        for j in (i + 1)..(i + n + 1) {
            let y = v[i];
            if let Some(x) = v.get_mut(j){
                *x += y;
            }
        }
    }
    v.iter().sum()
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (_, scratch_cards) = multi::separated_list1(newline, scratch_card_parser)(&input).unwrap();
    println!("day4a: {}", scratch_cards.iter().map(calc_winning).sum::<u32>());
    println!("day4b: {}", calc_cards(&scratch_cards));
}
