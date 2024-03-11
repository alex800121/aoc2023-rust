use std::collections::HashSet;

use project_root::get_project_root;

const N: usize = 131;

type M<T> = [[T; N]; N];

type Index = (isize, isize);
const ADJACENT: [Index; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
fn bfs(m: &M<char>, mut start: HashSet<(usize, usize)>) -> M<Option<usize>> {
    let mut output = [[None; N]; N];
    let mut visited = HashSet::new();
    let mut next_start = HashSet::new();
    let mut n = 0;
    while !start.is_empty() {
        for i @ (x, y) in start.drain() {
            visited.insert(i);
            output[y][x] = Some(n);
            for (a, b) in ADJACENT.iter() {
                let x = x as isize + a;
                let y = y as isize + b;
                if x >= 0
                    && y >= 0
                    && x < N as isize
                    && y < N as isize
                    && m[y as usize][x as usize] != '#'
                    && !visited.contains(&(x as usize, y as usize))
                {
                    next_start.insert((x as usize, y as usize));
                }
            }
        }
        start = std::mem::take(&mut next_start);
        n += 1;
    }
    output
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: M<char> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let start = HashSet::from_iter(input.iter().enumerate().find_map(|(y, xs)| {
        xs.iter()
            .enumerate()
            .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
    }));
    println!(
        "day21a: {}",
        bfs(&input, start)
            .iter()
            .map(|x| x
                .iter()
                .filter(|&&c| c.map(|c| c <= 64 && c % 2 == 0).unwrap_or(false))
                .count())
            .sum::<usize>()
    );
}
