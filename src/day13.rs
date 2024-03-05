use core::fmt::Debug;
use aoc2023::Transpose;
use project_root::get_project_root;

fn calc_symmetry<T: Eq + Debug>(input: &[Vec<T>], n: usize) -> Option<usize> {
    let l = input.len();
    (0..l).find(|i| {
        let mut j = 0;
        let mut m = 0;
        'a: for a in 0.. {
            if a > *i {
                break 'a;
            }
            j = i - a;
            let k = i + 1 + a;
            if let (Some(x), Some(y)) = (input.get(j), input.get(k)) {
                let l = x
                    .iter()
                    .zip(y)
                    .map(|(a, b)| if a == b { 0 } else { 1 })
                    .sum::<usize>();
                m += l
            } else {
                break 'a;
            }
        }
        j < l - 1 && m == n
    })
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let a = input
        .iter()
        .filter_map(|x| calc_symmetry(x, 0).map(|x| x + 1))
        .sum::<usize>();
    let mut input0 = input.clone();
    let b = input0
        .iter_mut()
        .filter_map(|x| calc_symmetry(&x.transpose(), 0).map(|x| x + 1))
        .sum::<usize>();
    println!("day13a: {}", 100 * a + b);
    let a = input
        .iter()
        .filter_map(|x| calc_symmetry(x, 1).map(|x| x + 1))
        .sum::<usize>();
    let mut input0 = input.clone();
    let b = input0
        .iter_mut()
        .filter_map(|x| calc_symmetry(&x.transpose(), 1).map(|x| x + 1))
        .sum::<usize>();
    println!("day13b: {}", 100 * a + b);
}
