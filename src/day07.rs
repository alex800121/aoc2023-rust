use nom::AsChar;
use project_root::get_project_root;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CardType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
type Hand = [u32; 5];
type Bid = (Hand, u32);

fn hand_type_b(hand: &Hand) -> CardType {
    use CardType::*;
    let mut v = [0; 15];
    let mut j = 0;
    for i in hand {
        if *i == 11 {
            j += 1;
        } else {
            v[*i as usize] += 1;
        }
    }
    let mut v: Vec<_> = v.into_iter().filter(|x| *x != 0).collect();
    v.sort();
    v.reverse();
    if let Some(x) = v.get_mut(0) {
        *x += j;
    }
    // dbg!(&v);
    match v[0..] {
        [5] => FiveOfAKind,
        [] => FiveOfAKind,
        [4, 1] => FourOfAKind,
        [3, 2] => FullHouse,
        [3, 1, 1] => ThreeOfAKind,
        [2, 2, 1] => TwoPair,
        [2, 1, 1, 1] => Pair,
        [1, 1, 1, 1, 1] => HighCard,
        _ => unreachable!(),
    }
}
fn hand_type(hand: &Hand) -> CardType {
    use CardType::*;
    let mut v = [0; 15];
    for i in hand {
        v[*i as usize] += 1;
    }
    let mut v: Vec<_> = v.iter().filter(|x| **x != 0).collect();
    v.sort();
    v.reverse();
    match v[0..] {
        [5] => FiveOfAKind,
        [4, 1] => FourOfAKind,
        [3, 2] => FullHouse,
        [3, 1, 1] => ThreeOfAKind,
        [2, 2, 1] => TwoPair,
        [2, 1, 1, 1] => Pair,
        [1, 1, 1, 1, 1] => HighCard,
        _ => unreachable!(),
    }
}
fn read_bid(input: &str) -> Bid {
    let v = input[0..5]
        .chars()
        .map(|c| read_card(&c))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let x = input[6..].parse().unwrap();
    (v, x)
}
fn read_card(input: &char) -> u32 {
    match input {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        x if x.is_dec_digit() => x.to_digit(10).unwrap(),
        _ => unreachable!(),
    }
}

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut bids = input.lines().map(|s| read_bid(&s)).collect::<Vec<_>>();
    bids.sort_by(|(a, _), (b, _)| hand_type(&a).cmp(&hand_type(&b)).then(a.cmp(&b)));
    let day7a = bids
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum::<u32>();
    println!("day7a: {}", day7a);
    bids.sort_by(|(a, _), (b, _)| {
        let a0 = a.map(|x| if x == 11 {1} else {x});
        let b0 = b.map(|x| if x == 11 {1} else {x});
        hand_type_b(&a).cmp(&hand_type_b(&b)).then(a0.cmp(&b0))
    });
    let day7b = bids
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum::<u32>();
    println!("day7b: {}", day7b);
}
