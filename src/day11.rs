use aoc2023::build_map;
use project_root::get_project_root;

type Stars = Vec<Index>;
type Lines = Vec<usize>;
type Index = (usize, usize);
fn manhattan_x(n: usize, xs: &Lines, ys: &Lines, i: &Index, j: &Index) -> usize {
    let a = i.0.min(j.0);
    let b = i.0.max(j.0);
    let o = b - a;
    let c = i.1.min(j.1);
    let d = i.1.max(j.1);
    let p = d - c;
    let xs = xs.iter().skip_while(|x| *x <= &a).take_while(|x| *x <= &b);
    let ys = ys.iter().skip_while(|x| *x <= &c).take_while(|x| *x <= &d);
    let x = xs.count();
    let y = ys.count();
    n * ((o - x) + (p - y)) + x + y
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let stars: Stars = Vec::from_iter(
        build_map(input.lines().map(|x| x.chars()), |k, a| {
            (if a == '#' { Some((k, ())) } else { None }).into_iter()
        })
        .into_iter()
        .map(|x| x.0),
    );
    let l = stars.len();
    let mut xs = stars.iter().map(|x| x.0).collect::<Lines>();
    let mut ys = stars.iter().map(|x| x.1).collect::<Lines>();
    xs.sort();
    ys.sort();
    xs.dedup();
    ys.dedup();
    // let (min_x, max_x, min_y, max_y) = stars
    //     .iter()
    //     .fold((100, 0, 100, 0), |(a, b, c, d), &(x, y)| {
    //         (a.min(x), b.max(x), c.min(y), d.max(y))
    //     });
    let day11a = {
        let mut sum = 0;
        for i in 0..l {
            for j in (i + 1)..l {
                sum += (|| {
                    let a = stars.get(i)?;
                    let b = stars.get(j)?;
                    let c = manhattan_x(2, &xs, &ys, a, b);
                    // dbg!(a, b, c);
                    Some(c)
                })().unwrap_or(0);
            }
        }
        sum
    };
    let day11b = {
        let mut sum = 0;
        for i in 0..l {
            for j in (i + 1)..l {
                sum += (|| {
                    let a = stars.get(i)?;
                    let b = stars.get(j)?;
                    let c = manhattan_x(1000000, &xs, &ys, a, b);
                    // dbg!(a, b, c);
                    Some(c)
                })().unwrap_or(0);
            }
        }
        sum
    };
    println!("day11a: {}", day11a);
    println!("day11b: {}", day11b);
}
