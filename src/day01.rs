use std::string;

use project_root::get_project_root;

const NUM : [[&str; 2]; 10] = [
    ["0", "zero"],
    ["1", "one"],
    ["2", "two"],
    ["3", "three"],
    ["4", "four"],
    ["5", "five"],
    ["6", "six"],
    ["7", "seven"],
    ["8", "eight"],
    ["9", "nine"]
];
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let day1a = &input
        .lines()
        .filter_map(|x| {
            let mut y = x.chars().filter(|c| c.is_ascii_digit());
            let h = y.next()?;
            let l = y.last().unwrap_or(h);
            let mut output = string::String::new();
            output.push(h);
            output.push(l);
            output.parse::<usize>().ok()
        }).sum::<usize>();
    let day1b = &input
        .lines()
        .filter_map(|x| {
            let mut y = String::new();
            let mut x = x.chars();
            while x.as_str() != "" {
                for (i, s) in NUM.iter().enumerate() {
                    if s.iter().any(|&c| x.as_str().starts_with(c)) {
                        y.push_str(&format!("{}",  i));
                    }
                }
                x.next();
            }
            let mut output = String::new();
            let mut y = y.chars();
            let h = y.next()?;
            let l = y.last().unwrap_or(h);
            output.push(h);
            output.push(l);
            output.parse::<usize>().ok()
        }).sum::<usize>();
    println!("day1a: {}", day1a);
    println!("day1b: {}", day1b);
}
