use std::collections::HashSet;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let (mut hx, mut hy): (i16, i16) = (0, 0);
    let (mut tx, mut ty): (i16, i16)  = (0, 0);
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
            hx += dir.0;
            hy += dir.1;
            let (dx, dy) = (hx-tx, hy-ty);
            match (dx, dy) {
                (2, 0) => { tx += 1; },
                (-2, 0) => { tx -= 1; },
                (0, 2) => { ty += 1; },
                (0, -2) => { ty -= 1; },
                _ if dx.abs() + dy.abs() <= 2 => {},
                _ => { tx += dx.signum(); ty += dy.signum() },
            };
            visited.insert((tx, ty));
        }
    });
    println!("{}", visited.len());
}
