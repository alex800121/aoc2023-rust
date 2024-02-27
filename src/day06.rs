use aoc2023::ZipWith;
use project_root::get_project_root;

// The race lasts n sec, record dist
// holds x sec, run (n - x) sec with a speed of x
// dist = (n - x) * x
//      = -x*x + n*x
//      = -(x - (n/2))^2 + ((n^2)/4)
// x*x + (-n)*x + dist = 0

type Races = (f64, f64);
fn solve_race((sec, rec): Races) -> u64 {
    let b2ac4 = (sec * sec - 1.0 * rec * 4.0).sqrt();
    let a2 = 2.0;
    let bneg = sec;
    let (low, high) = ((bneg - b2ac4) / a2, (bneg + b2ac4) / a2);
    ((high - 1.0).ceil() - (low + 1.0).floor()) as u64 + 1
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut a = input.lines().map(|x| {
        x.split_whitespace()
            .filter_map(|y| y.parse::<f64>().ok())
            .collect::<Vec<f64>>()
    });
    let races = (|| {
        let x = a.next()?;
        let y = a.last()?;
        Some(x.zip_with(|a, b| (*a, *b), y))
    })()
    .unwrap_or(Vec::new());
    let (x, y) = races.iter().fold((String::new(), String::new()), |(x, y), (a, b)| {
        (format!("{}{}", x, *a as u64), format!("{}{}", y, *b as u64))
    });
    println!("day6a: {}", races.iter().map(|x| solve_race(*x)).product::<u64>());
    println!("day6b: {}", solve_race((x.parse().unwrap(), y.parse().unwrap())));
}
