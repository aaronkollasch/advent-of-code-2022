use std::collections::HashSet;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut rope: [(i16, i16); 10] = [(0, 0); 10];
    let mut visited: HashSet::<(i16, i16)> = HashSet::with_capacity(256);

    s.split(|b| *b == b'\n').filter(|l| l.len() >= 3).for_each(|l| {
        let dir = match l[0] {
            b'U' => (0, 1),
            b'D' => (0, -1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => (0, 0),
        };
        let dist = l[2..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u8);
        for _i in 0..dist {
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            for j in 1..10 {
                let (dx, dy) = (rope[j-1].0-rope[j].0, rope[j-1].1-rope[j].1);
                match (dx, dy) {
                    (2, 0) => { rope[j].0 += 1; },
                    (-2, 0) => { rope[j].0 -= 1; },
                    (0, 2) => { rope[j].1 += 1; },
                    (0, -2) => { rope[j].1 -= 1; },
                    _ if dx.abs() + dy.abs() <= 2 => {},
                    _ => { rope[j].0 += dx.signum(); rope[j].1 += dy.signum() },
                };
            }
            visited.insert(rope[9]);
        }
    });
    println!("{}", visited.len());
}
