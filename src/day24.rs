use itertools::Itertools;
use nalgebra::{matrix, SMatrix};
use num::{rational::Ratio, BigInt, BigRational, FromPrimitive};
use project_root::get_project_root;

/*
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4

t0 = (x0 - a0) / ax0

a0 * by0 - b0 * ax0 = by0 * x - ax0 * y

a1 + ax1 * t1 = x1
b1 + by1 * t1 = y1

by0   (-ax0)        x       a0 * by0 - b0 * ax0
                X       =
by1   (-ax1)        y       a1 * by1 - b1 * ax1
--------------------------------------------------
a0 + aa0 * t0 = x + xx * t0
b0 + bb0 * t0 = y + yy * t0
c0 + cc0 * t0 = z + zz * t0
a1 + aa1 * t1 = x + xx * t1
b1 + bb1 * t1 = y + yy * t1
c1 + cc1 * t1 = z + zz * t1
a2 + aa2 * t2 = x + xx * t2
b2 + bb2 * t2 = y + yy * t2
c2 + cc2 * t2 = z + zz * t2
(a0 - x) * (yy - bb0) = (b0 - y) * (xx - aa0)
a0 * yy + bb0 * x - b0 * x - aa0 * y - b0 * xx = a0 * bb0 + x * yy - b0 * aa0 - y * xx
a1 * yy + bb1 * x - b1 * x - aa1 * y - b1 * xx = a1 * bb1 + x * yy - b1 * aa1 - y * xx
a2 * yy + bb2 * x - b2 * x - aa2 * y - b2 * xx = a2 * bb2 + x * yy - b2 * aa2 - y * xx
(a0 - a1) * yy + (bb0 - bb1) * x - (aa0 - aa1) * y - (b0 - b1) * xx = a0 * bb0 - b0 * aa0 - a1 * bb1 + b1 * aa1
(a1 - a2) * yy + (bb1 - bb2) * x - (aa1 - aa2) * y - (b1 - b2) * xx = a1 * bb1 - b1 * aa1 - a2 * bb2 + b2 * aa2
(b0 - b1) * zz + (cc0 - cc1) * y - (bb0 - bb1) * z - (c0 - c1) * yy = b0 * cc0 - c0 * bb0 - b1 * cc1 + c1 * bb1
(b1 - b2) * zz + (cc1 - cc2) * y - (bb1 - bb2) * z - (c1 - c2) * yy = b1 * cc1 - c1 * bb1 - b2 * cc2 + c2 * bb2
(c0 - c1) * xx + (aa0 - aa1) * z - (cc0 - cc1) * x - (a0 - a1) * zz = c0 * aa0 - a0 * cc0 - c1 * aa1 + a1 * cc1
(c1 - c2) * xx + (aa1 - aa2) * z - (cc1 - cc2) * x - (a1 - a2) * zz = c1 * aa1 - a1 * cc1 - c2 * aa2 + a2 * cc2
*/

type Index<T> = (T, T, T);
type Asteroid<T> = (Index<T>, Index<T>);
type AsteroidBig = Asteroid<BigRational>;
const LOW: f64 = 200000000000000.0;
const HIGH: f64 = 400000000000000.0;

fn g_convert(m: &mut SMatrix<BigRational, 6, 12>) {
    for i in 0..6 {
        let x = m[(i, i)].clone();
        for j in 0..6 {
            if i != j {
                let y = m[(j, i)].clone();
                let z = y / x.clone();
                for k in 0..12 {
                    let a = m[(i, k)].clone();
                    m[(j, k)] -= z.clone() * a;
                }
            }
        }
        for k in 0..12 {
            m[(i, k)] /= x.clone();
        }
    }
}
fn solve_b(
    ((a0, b0, c0), (aa0, bb0, cc0)): &AsteroidBig,
    ((a1, b1, c1), (aa1, bb1, cc1)): &AsteroidBig,
    ((a2, b2, c2), (aa2, bb2, cc2)): &AsteroidBig,
) -> Option<(BigRational, BigRational, BigRational)> {
    // (a0 - a1) * yy + (bb0 - bb1) * x - (aa0 - aa1) * y - (b0 - b1) * xx = a0 * bb0 - b0 * aa0 - a1 * bb1 + b1 * aa1
    // (a1 - a2) * yy + (bb1 - bb2) * x - (aa1 - aa2) * y - (b1 - b2) * xx = a1 * bb1 - b1 * aa1 - a2 * bb2 + b2 * aa2
    // (b0 - b1) * zz + (cc0 - cc1) * y - (bb0 - bb1) * z - (c0 - c1) * yy = b0 * cc0 - c0 * bb0 - b1 * cc1 + c1 * bb1
    // (b1 - b2) * zz + (cc1 - cc2) * y - (bb1 - bb2) * z - (c1 - c2) * yy = b1 * cc1 - c1 * bb1 - b2 * cc2 + c2 * bb2
    // (c0 - c1) * xx + (aa0 - aa1) * z - (cc0 - cc1) * x - (a0 - a1) * zz = c0 * aa0 - a0 * cc0 - c1 * aa1 + a1 * cc1
    // (c1 - c2) * xx + (aa1 - aa2) * z - (cc1 - cc2) * x - (a1 - a2) * zz = c1 * aa1 - a1 * cc1 - c2 * aa2 + a2 * cc2
    let zero: BigRational = BigRational::from_integer(BigInt::from(0));
    let one: BigRational = BigRational::from_integer(BigInt::from(1));
    let mut m1: SMatrix<_, 6, 12> = matrix![bb0 - bb1, aa1 - aa0, zero.clone(), b1 - b0, a0 - a1, zero.clone(), one.clone(), zero.clone(), zero.clone(), zero.clone(), zero.clone(), zero.clone();
                bb1 - bb2, aa2 - aa1, zero.clone(), b2 - b1, a1 - a2, zero.clone(), zero.clone(), one.clone(), zero.clone(), zero.clone(), zero.clone(), zero.clone();
                zero.clone(), cc0 - cc1, bb1 - bb0, zero.clone(), c1 - c0, b0 - b1, zero.clone(), zero.clone(), one.clone(), zero.clone(), zero.clone(), zero.clone();
                zero.clone(), cc1 - cc2, bb2 - bb1, zero.clone(), c2 - c1, b1 - b2, zero.clone(), zero.clone(), zero.clone(), one.clone(), zero.clone(), zero.clone();
                cc1 - cc0, zero.clone(), aa0 - aa1, c0 - c1, zero.clone(), a1 - a0, zero.clone(), zero.clone(), zero.clone(), zero.clone(), one.clone(), zero.clone();
                cc2 - cc1, zero.clone(), aa1 - aa2, c1 - c2, zero.clone(), a2 - a1, zero.clone(), zero.clone(), zero.clone(), zero.clone(), zero.clone(), one;];
    let m2: SMatrix<_, 6, 1> = matrix![a0 * bb0 - b0 * aa0 - a1 * bb1 + b1 * aa1;
                a1 * bb1 - b1 * aa1 - a2 * bb2 + b2 * aa2;
                b0 * cc0 - c0 * bb0 - b1 * cc1 + c1 * bb1;
                b1 * cc1 - c1 * bb1 - b2 * cc2 + c2 * bb2;
                c0 * aa0 - a0 * cc0 - c1 * aa1 + a1 * cc1;
                c1 * aa1 - a1 * cc1 - c2 * aa2 + a2 * cc2;];
    g_convert(&mut m1);
    let m3: SMatrix<BigRational, 6, 6> = SMatrix::from_fn(|r, c| m1[(r, c + 6)].clone());
    let c = m3 * m2;
    Some((c[(0, 0)].clone(), c[(1, 0)].clone(), c[(2, 0)].clone()))
}
fn solve_a(
    ((a0, b0, _), (ax0, by0, _)): Asteroid<f64>,
    ((a1, b1, _), (ax1, by1, _)): Asteroid<f64>,
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
    let input: Vec<Asteroid<f64>> = input
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
                    if (LOW..=HIGH).contains(&x)
                        && (LOW..=HIGH).contains(&y)
                        && t0 > 0.0
                        && t1 > 0.0
                    {
                        ans_a += 1;
                    }
                }
            }
        }
    }
    // dbg!(&input);
    // dbg!(solve_a(*input.first().unwrap(), *input.get(1).unwrap()));
    println!("day24a: {}", ans_a);
    let input = input.into_iter().map(|((a, b, c), (d, e, f))| {
        let [a, b, c, d, e, f] = [a, b, c, d, e, f].map(|x| BigRational::from_f64(x).unwrap());
        ((a, b, c), (d, e, f))
    }).collect_vec();
    let (a, b, c) = solve_b(
        input.first().unwrap(),
        input.get(1).unwrap(),
        input.get(2).unwrap(),
    ).unwrap();
    println!("day24b: {}", a.to_integer() + b.to_integer() + c.to_integer());
}
