use common::parse_signed;
use std::collections::VecDeque;

type Number = isize;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut numbers: VecDeque<(Number, bool)> = VecDeque::with_capacity(8192);
    numbers.extend(
        s.split(|b| *b == b'\n')
            .filter(|l| !l.is_empty())
            .map(|l| (parse_signed::<Number>(l), false)),
    );
    let mut num_moves = 0;
    let mut index = 0;
    let len = numbers.len();
    while num_moves < numbers.len() {
        while numbers[index].1 {
            index = (index + 1) % len;
        }
        #[cfg(debug_assertions)]
        println!(
            "{:?} {index}",
            numbers.iter().map(|t| t.0).collect::<Vec<_>>()
        );
        let (amount, _) = numbers[index];
        let new_index = (index as Number + amount).rem_euclid((len - 1) as Number) as usize;
        #[cfg(debug_assertions)]
        println!("move {} ({}) -> {}", index, amount, new_index);
        numbers.remove(index);
        numbers.insert(new_index, (amount, true));
        num_moves += 1;
    }
    let zpos = numbers.iter().position(|(n, _i)| *n == 0).unwrap();
    #[cfg(debug_assertions)]
    println!(
        "{} {} {}",
        numbers[(zpos + 1000) % len].0,
        numbers[(zpos + 2000) % len].0,
        numbers[(zpos + 3000) % len].0
    );
    let result = [zpos + 1000, zpos + 2000, zpos + 3000]
        .iter()
        .map(|n| numbers[n % len].0)
        .sum::<Number>();
    print!("{} ", result);
}
