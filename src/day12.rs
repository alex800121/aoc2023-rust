use project_root::get_project_root;

fn input_parser(input: &str) -> Option<(&str, Vec<usize>)> {
    let (x, y) = input.split_once(' ')?;
    let y = y
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();
    Some((x, y))
}
fn unfold(input: &str) -> String {
    let input = format!("{}?", input).repeat(5);
    input[0..(input.len() - 1)].to_string()
}
fn solve(input: &str, counts: &Vec<usize>) -> usize {
    let input = format!("..{}", input).chars().collect::<Vec<_>>();
    let l = input.len();
    let mut v0 = vec![0; l];
    for (i, &c) in input.iter().enumerate() {
        if c == '#' {
            break;
        }
        v0[i] = 1;
    }
    for count in counts {
        let mut v1 = vec![0; l];
        let mut chunks = 0;
        for (i, &c) in input.iter().enumerate() {
            if c == '.' {
                chunks = 0;
            } else {
                chunks += 1;
            }
            if i > 0 && c != '#' {
                v1[i] = v1[i - 1]
            }
            if chunks >= *count && i > *count && input[i-count] != '#' {
                v1[i] += v0[i - count - 1];
            }
        }
        v0 = v1;
    }
    v0[l - 1]
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.lines().filter_map(input_parser).collect::<Vec<_>>();
    let a = input.iter().map(|(x, y)| solve(x, y)).sum::<usize>();
    println!("day12a: {a}");
    let b = input.iter().map(|(x, y)| solve(&unfold(x), &y.repeat(5))).sum::<usize>();
    println!("day12b: {b}");
}
