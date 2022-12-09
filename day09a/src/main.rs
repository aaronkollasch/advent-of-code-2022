use std::collections::HashSet;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let (mut hx, mut hy): (i16, i16) = (0, 0);
    let (mut tx, mut ty): (i16, i16)  = (0, 0);
    let mut visited: HashSet::<(i16, i16)> = HashSet::with_capacity(8192);

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
            if hx.abs_diff(tx).max(hy.abs_diff(ty)) > 1 {
                tx += (hx - tx).signum();
                ty += (hy - ty).signum();
            }
            visited.insert((tx, ty));
        }
    });
    #[cfg(debug_assertions)]
    println!("{} {}", visited.iter().map(|k| k.0.abs()).max().unwrap(), visited.iter().map(|k| k.1.abs()).max().unwrap());
    println!("{}", visited.len());
}
