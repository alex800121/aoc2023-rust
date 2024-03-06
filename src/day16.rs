use std::collections::BTreeSet;

use aoc2023::{
    Direction::{self, *},
    Enum,
};
use project_root::get_project_root;

type Bot = (Index, Direction);

type Index = (usize, usize);
type M = [[char; 110]; 110];

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
struct GameState {
    visited: BTreeSet<Index>,
    bots: Vec<Bot>,
}

fn reflect(c: char, d: Direction) -> Vec<Direction> {
    match (c, d) {
        ('.', x) => vec![x],
        ('|', East) => vec![North, South],
        ('|', West) => vec![North, South],
        ('|', x) => vec![x],
        ('-', North) => vec![East, West],
        ('-', South) => vec![East, West],
        ('-', x) => vec![x],
        ('/', x) if x.to_int() % 2 == 0 => vec![x.succ()],
        ('/', x) => vec![x.pred()],
        ('\\', x) if x.to_int() % 2 == 0 => vec![x.pred()],
        ('\\', x) => vec![x.succ()],
        _ => panic!("wrong char"),
    }
}

fn run_game(g: &mut GameState, m: &M) {
    let mut visited_bot: BTreeSet<Bot> = BTreeSet::new();
    let y_len = m.len() as isize;
    let x_len = m[0].len() as isize;
    let mut new_bot = Vec::new();
    while !g.bots.is_empty() {
        for b in g.bots.drain(..) {
            visited_bot.insert(b);
            g.visited.insert(b.0);
            let c = m[b.0 .1][b.0 .0];
            for new_d in reflect(c, b.1) {
                let (x, y) = new_d.to_index();
                let (new_x, new_y) = (b.0 .0 as isize + x, b.0 .1 as isize + y);
                if new_x >= 0 && new_x < x_len && new_y >= 0 && new_y < y_len {
                    let nb = ((new_x as usize, new_y as usize), new_d);
                    if !visited_bot.contains(&nb) {
                        new_bot.push(nb);
                    }
                }
            }
        }
        g.bots = std::mem::take(&mut new_bot);
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: M = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut g = GameState {
        visited: BTreeSet::new(),
        bots: vec![((0, 0), East)],
    };
    run_game(&mut g, &input);
    println!("day16a: {}", g.visited.len());
    let b = (0..110).flat_map(|a| {
        [
            ((0, a), East),
            ((109, a), West),
            ((a, 0), South),
            ((a, 109), North),
        ]
        .map(|x| {
            let mut g = GameState {
                visited: BTreeSet::new(),
                bots: vec![x],
            };
            run_game(&mut g, &input);
            g.visited.len()
        })
    }).max();
    println!("day16b: {}", b.unwrap());
}
