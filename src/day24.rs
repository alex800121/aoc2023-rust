use itertools::Itertools;
use nalgebra::{matrix, SMatrix};
use project_root::get_project_root;

/*

19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4

a0 + ax0 * t0 = x0
b0 + by0 * t0 = y0
c0 + cz0 * t0 = z0

a1 + ax1 * t1 = x1
b1 + by1 * t1 = y1
c1 + cz1 * t1 = z1

a2 + ax2 * t2 = x2
b2 + by2 * t2 = y2
c2 + cz2 * t2 = z2

a1' + ax1' * t1' = x1
b1' + by1' * t1' = y1
c1' + cz1' * t1' = z1
a2' + ax2' * t2' = x2
b2' + by2' * t2' = y2
c2' + cz2' * t2' = z2
(a1' + ax1' * t1') * (b2' + by2' * t2') = (a2' + ax2' * t2') * (b1' + by1' * t1')
(a1' * b2' - a2' * b1') + t1' * (ax1' * b2' - by1' - a2') + t2' * 

t0 = (x0 - a0) / ax0

a0 * by0 - b0 * ax0 = by0 * x - ax0 * y

a1 + ax1 * t1 = x1
b1 + by1 * t1 = y1

by0   (-ax0)        x       a0 * by0 - b0 * ax0
                X       =
by1   (-ax1)        y       a1 * by1 - b1 * ax1

*/

type Index = (f64, f64, f64);
type Asteroid = (Index, Index);
const LOW: f64 = 200000000000000.0;
const HIGH: f64 = 400000000000000.0;

fn solve_a(
    ((a0, b0, _), (ax0, by0, _)): Asteroid,
    ((a1, b1, _), (ax1, by1, _)): Asteroid,
) -> Option<(f64, f64, f64, f64)> {
    let a: SMatrix<_, 2, 2> = matrix![by0, (-ax0);
                                      by1, (-ax1)];
    let b: SMatrix<_, 2, 1> = matrix![(a0 * by0 - b0 * ax0);
                                      (a1 * by1 - b1 * ax1)];
    // dbg!(a);
    let a = a.try_inverse()?;
    let c = a * b;
    // dbg!(c);
    let x = c[(0, 0)];
    let y = c[(1, 0)];
    let t0 = (x - a0) / ax0;
    let t1 = (x - a1) / ax1;
    Some((x, y, t0, t1))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: Vec<Asteroid> = input
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once(" @ ")?;
            let [x0, y0, z0] = a
                .split(", ")
                .filter_map(|x| x.trim().parse::<f64>().ok())
                .collect_vec()
                .try_into()
                .ok()?;
            let [vx, vy, vz] = b
                .split(", ")
                .filter_map(|x| x.trim().parse::<f64>().ok())
                .collect_vec()
                .try_into()
                .ok()?;
            Some(((x0, y0, z0), (vx, vy, vz)))
        })
        .collect_vec();
    let mut ans_a = 0;
    for x in 0..input.len() {
        for y in (x + 1)..input.len() {
            if let (Some(a), Some(b)) = (input.get(x), input.get(y)) {
                if let Some((x, y, t0, t1)) = solve_a(*a, *b) {
                    if (LOW..=HIGH).contains(&x) && (LOW..=HIGH).contains(&y) && t0 > 0.0 && t1 > 0.0 {
                        ans_a += 1;
                    }
                }
            }
        }
    }
    // dbg!(&input);
    // dbg!(solve_a(*input.first().unwrap(), *input.get(1).unwrap()));
    println!("day24a: {}", ans_a);
}
