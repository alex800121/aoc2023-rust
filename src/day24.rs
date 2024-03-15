use itertools::Itertools;
use project_root::get_project_root;

// 206273907288897, 404536114337943, 197510451330134 @ -18, 6, 92
// 318919383845607, 260745469021671, 223155534318195 @ -78, 62, 75
// 379055259398812, 255495760772511, 396757430832289 @ -179, -18, -373

type Index = (f64, f64, f64);
type Asteroid = (Index, Index);

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: Vec<Asteroid> = input
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once(" @ ")?;
            let [x0, y0, z0] = a.split(", ").filter_map(|x| x.parse::<f64>().ok()).collect_vec().try_into().ok()?;
            let [vx, vy, vz] = b.split(", ").filter_map(|x| x.parse::<f64>().ok()).collect_vec().try_into().ok()?;
            Some(((x0, y0, z0), (vx, vy, vz)))
        }).collect_vec();
    dbg!(input);
}
