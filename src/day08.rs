use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use nom::character::complete::char;
use num::Integer;
use project_root::get_project_root;

use std::collections::HashMap;
use std::collections::HashSet;
type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct GameState<'a> {
    instruction: usize,
    node: &'a str,
}
type Visited<'a> = HashSet<GameState<'a>>;
fn next_gamestate<'a, const N: usize>(
    ins: &[bool; N],
    node_map: &NodeMap<'a>,
    g: &GameState<'a>,
) -> GameState<'a> {
    let f = |(x, y)| {
        if ins[g.instruction] {
            x
        } else {
            y
        }
    };
    GameState {
        instruction: (g.instruction + 1).mod_floor(&N),
        node: f(*node_map.get(g.node).unwrap()),
    }
}
const RAWINS: &str =  "LRRRLRRLRRLRRLLLRRRLRRLLRRRLRLLLRRLRLRLRLRLRLRLRRRLLLRRLRRRLRLLRRRLRRRLRRRLLRRRLRLRRRLRRLRRRLLRLLRLLRRRLRRRLRRLRLRLLRLRRLRRRLRRRLRLRLRLRRLRLRLLLRRRLRLRLRRRLRRRLRRRLRLLLRRLRLRLRLRLLLRRRLRRLRRLRLRLRRRLRLRRRLRRRLRRRLRLRRRLLLRRLRRRLRRLLRLRRLRRLRRRLLLRRLRRLRRLRLRRRLLLRLRRRR";
const LEN: usize = RAWINS.len();
fn day8a<'a, const N: usize, T>(
    ins: &[bool; N],
    node_map: &NodeMap<'a>,
    start: &GameState<'a>,
    end: T,
) -> (&'a str, usize)
where
    T: Fn(&'a str) -> bool,
{
    let mut g = *start;
    let mut n = 0;
    while !end(g.node) {
        g = next_gamestate(ins, node_map, &g);
        n += 1;
    }
    (g.node, n)
}

fn node_parser(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, a) = terminated(alpha1, tag(" = "))(input)?;
    let (input, (b, c)) = delimited(
        char('('),
        separated_pair(alpha1, tag(", "), alpha1),
        char(')'),
    )(input)?;
    Ok((input, (a, (b, c))))
}
pub fn run(day: usize) {
    let ins: [bool; LEN] = RAWINS
        .chars()
        .map(|x| x == 'L')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let node_map: NodeMap = HashMap::from_iter(input.lines().filter_map(|s| node_parser(s).ok().map(|(_, x)| x)));
    let g = GameState {
        instruction: 0,node: "AAA"
    };
    let (_, a) = day8a(&ins, &node_map, &g, |x| x == "ZZZ");
    println!("day8a: {}", a);
}
