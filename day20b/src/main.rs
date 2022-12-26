use std::collections::VecDeque;
// TODO: see https://www.reddit.com/r/adventofcode/comments/zqezkn/comment/j0yaksg
// TODO: see https://github.com/p88h/aoc2022/blob/main/lib/day20.cs
// TODO: see https://github.com/orlp/aoc2022/blob/master/src/treap.rs
use common::parse_signed;

type Number = isize;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut numbers: VecDeque<(Number, usize)> = VecDeque::with_capacity(32);
    s.split(|b| *b == b'\n').filter(|l| !l.is_empty()).enumerate().for_each(|(i, l)| {
        numbers.push_back((parse_signed::<Number>(l) * 811589153, i));
    });
    let len = numbers.len();
    for _round in 0..10 {
        for idx in 0..len {
            let index = numbers.iter().position(|(_n, i)| *i == idx).unwrap();
            #[cfg(debug_assertions)]
            println!("{:?} {index}", numbers.iter().map(|t| t.0).collect::<Vec<_>>());
            let (amount, _) = numbers[index];
            let new_index = index as Number + amount;
            let new_index = new_index.rem_euclid((len - 1) as Number) as usize;
            #[cfg(debug_assertions)]
            println!("move {} ({}) -> {}", index, amount, new_index);
            let v = numbers.remove(index).unwrap();
            numbers.insert(new_index, v);
        }
    }
    let zpos = numbers.iter().position(|(n, _i)| *n == 0).unwrap();
    #[cfg(debug_assertions)]
    println!("{} {} {}", numbers[(zpos + 1000) % len].0, numbers[(zpos + 2000) % len].0, numbers[(zpos + 3000) % len].0);
    let result = [
        zpos + 1000, zpos + 2000, zpos + 3000
    ].iter().map(|n| numbers[n % len].0).sum::<Number>();
    print!("{} ", result);
}
