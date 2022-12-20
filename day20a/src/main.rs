use std::{collections::HashSet, cmp::Ordering};

type Number = isize;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut numbers: Vec<(Number, bool)> = Vec::with_capacity(32);
    s.split(|b| *b == b'\n').filter(|l| !l.is_empty()).for_each(|l| {
        let mut acc = 0;
        let mut sign = 1;
        for b in l.iter() {
            match b {
                b'-' => {
                    sign = -1;
                }
                b'0'..=b'9' => {
                    acc = acc * 10 + (b - b'0') as Number;
                }
                _ => {}
            }
        }
        numbers.push((sign * acc, false));
    });
    let numbers_unique: HashSet<Number> = HashSet::from_iter(numbers.iter().map(|(a, _)| *a));
    #[cfg(debug_assertions)]
    println!("num numbers {}, unique {}", numbers.len(), numbers_unique.len());
    let mut num_moves = 0;
    let mut index = 0;
    let len = numbers.len();
    while num_moves < numbers.len() {
        while numbers[index].1 {
            index = (index + 1) % len;
        }
        #[cfg(debug_assertions)]
        println!("{:?} {index}", numbers.iter().map(|t| t.0).collect::<Vec<_>>());
        let (amount, _) = numbers[index];
        let mut new_index = index as Number + amount;
        // if new_index <= 0 {
        //     new_index -= 1;
        // } else if new_index >= len as Number - 1 {
        //     new_index += 1;
        // }
        let new_index = new_index.rem_euclid((len - 1) as Number) as usize;
        #[cfg(debug_assertions)]
        println!("move {} ({}) -> {}", index, amount, new_index);
        numbers.remove(index);
        numbers.insert(new_index, (amount, true));
        // if new_index <= index {
        //     index = (index + 1) % len;
        // }
        num_moves += 1;
    }
    // let s = include_str!("../input.txt");
    // s.lines().for_each(|l| {
    // });
    let zpos = numbers.iter().position(|(n, _i)| *n == 0).unwrap();
    #[cfg(debug_assertions)]
    println!("{} {} {}", numbers[(zpos + 1000) % len].0, numbers[(zpos + 2000) % len].0, numbers[(zpos + 3000) % len].0);
    let result = [
        zpos + 1000, zpos + 2000, zpos + 3000
    ].iter().map(|n| numbers[n % len].0).sum::<Number>();
    print!("{} ", result);
}
