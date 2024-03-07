use aoc2023::{
    Direction::{self, *},
    Enum,
};
use pathfinding::{directed::dijkstra, prelude::dijkstra};
use project_root::get_project_root;

type Index = (isize, isize);
type M = [[usize; 141]; 141];

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
struct IndexPlus {
    position: Index,
    direction: (Direction, usize),
}

fn success(i: &IndexPlus, m: &M) -> bool {
    let max_x = m.first().map(|x| x.len()).unwrap_or(0) as isize;
    let max_y = m.len() as isize;
    i.position == (max_x - 1, max_y - 1)
}
fn successors(
    start: &IndexPlus,
    m: &M,
    max_step: usize,
    min_step: usize,
) -> Vec<(IndexPlus, usize)> {
    let max_x = m[0].len() as isize;
    let max_y = m.len() as isize;
    [
        (start.direction.0.pred(), 1),
        (start.direction.0, start.direction.1 + 1),
        (start.direction.0.succ(), 1),
    ]
    .iter()
    .filter_map(|(d, n)| {
        let (mut x, mut y) = start.position;
        let (x1, y1) = d.to_index();
        let mut n = *n;
        x += x1;
        y += y1;
        let mut c = *m.get(y as usize).and_then(|y| y.get(x as usize))?;
        while n < min_step {
            x += x1;
            y += y1;
            n += 1;
            c += m.get(y as usize).and_then(|y| y.get(x as usize))?;
        }
        if n <= max_step && x >= 0 && x < max_x && y >= 0 && y < max_y {
            Some((
                IndexPlus {
                    position: (x, y),
                    direction: (*d, n),
                },
                c,
            ))
        } else {
            None
        }
    })
    .collect()
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
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let start1 = IndexPlus {
        position: (0, 0),
        direction: (South, 0),
    };
    let start2 = IndexPlus {
        position: (0, 0),
        direction: (East, 0),
    };
    let a1 = dijkstra(
        &start1,
        |x| successors(x, &input, 3, 0),
        |x| success(x, &input),
    );
    let a2 = dijkstra(
        &start2,
        |x| successors(x, &input, 3, 0),
        |x| success(x, &input),
    );
    println!("day17a: {}", a1.unwrap().1.min(a2.unwrap().1));
    let a1 = dijkstra(
        &start1,
        |x| successors(x, &input, 10, 4),
        |x| success(x, &input),
    );
    let a2 = dijkstra(
        &start2,
        |x| successors(x, &input, 10, 4),
        |x| success(x, &input),
    );
    println!("day17b: {}", a1.unwrap().1.min(a2.unwrap().1));
}
