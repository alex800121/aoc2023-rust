use std::u8;

use project_root::get_project_root;

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0u8, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17)) as usize
}
type LensBox<'a> = [Vec<Lens<'a>>; 256];
type Lens<'a> = (&'a str, usize);

fn with_ins<'a>(lens_box: &mut LensBox<'a>, ins: &'a str) {
    let mut ins = ins.split(|x| x == '=' || x == '-');
    let lens_tag = ins.next().unwrap();
    match ins.next().and_then(|x| x.parse::<usize>().ok()) {
        None => remove_lens(lens_box, lens_tag),
        Some(i) => {
            insert_lens(lens_box, &(lens_tag, i));
        }
    }
}
fn insert_lens<'a>(lens_box: &mut LensBox<'a>, lens: &Lens<'a>) {
    let h = hash(lens.0);
    if let Some(b) = lens_box.get_mut(h) {
        for x in b.iter_mut() {
            if x.0 == lens.0 {
                x.1 = lens.1;
                return;
            }
        }
        b.push(*lens);
    }
}
fn remove_lens<'a>(lens_box: &mut LensBox<'a>, lens: &'a str) {
    let h = hash(lens);
    if let Some(b) = lens_box.get_mut(h) {
        b.retain(|(s, _)| *s != lens);
    }
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let a = &input.trim_end().split(',').map(hash).sum::<usize>();
    println!("day15a: {}", a);
    let mut v: LensBox = std::array::from_fn(|_| Vec::new());
    for ins in input.trim_end().split(',') {
        with_ins(&mut v, ins);
    }
    println!(
        "day15b: {}",
        v.iter()
            .enumerate()
            .map(|(i, x)| (i + 1)
                * x.iter()
                    .enumerate()
                    .map(|(j, y)| (j + 1) * y.1)
                    .sum::<usize>())
            .sum::<usize>()
    );
}
