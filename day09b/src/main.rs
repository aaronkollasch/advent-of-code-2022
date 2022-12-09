use std::collections::HashSet;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut rope: [(i16, i16); 10] = [(0, 0); 10];
    let mut visited: HashSet<(i16, i16)> = HashSet::with_capacity(8192);

    s.split(|b| *b == b'\n')
        .filter(|l| l.len() >= 3)
        .for_each(|l| {
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
                    let head = rope[j - 1];
                    let tail = rope[j];
                    if head.0.abs_diff(tail.0).max(head.1.abs_diff(tail.1)) > 1 {
                        rope[j].0 += (head.0 - tail.0).signum();
                        rope[j].1 += (head.1 - tail.1).signum();
                    }
                }
                visited.insert(rope[9]);
            }
        });
    #[cfg(debug_assertions)]
    println!(
        "{} {}",
        visited.iter().map(|k| k.0.abs()).max().unwrap(),
        visited.iter().map(|k| k.1.abs()).max().unwrap()
    );
    println!("{}", visited.len());
}
