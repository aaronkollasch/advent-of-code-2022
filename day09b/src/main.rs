const ROPE_LEN: usize = 10;

pub fn main() {
    let cmds = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| {
            match (
                l[0],
                l[2..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u8),
            ) {
                (b'U', l) => ((0, -1), l),
                (b'D', l) => ((0, 1), l),
                (b'L', l) => ((-1, 0), l),
                (_, l) => ((1, 0), l),
            }
        });
    let mut rope: [(i32, i32); ROPE_LEN] = [(0, 0); ROPE_LEN];
    let mut visited: rustc_hash::FxHashSet<_> = Default::default();
    visited.reserve(8192);
    visited.insert((0, 0));

    for (dir, dist) in cmds {
        for _i_step in 0..dist {
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            for j in 1..ROPE_LEN {
                let head = rope[j - 1];
                let tail = rope[j];
                if head.0.abs_diff(tail.0).max(head.1.abs_diff(tail.1)) > 1 {
                    rope[j].0 += (head.0 - tail.0).signum();
                    rope[j].1 += (head.1 - tail.1).signum();
                    (j == ROPE_LEN - 1).then(|| visited.insert(rope[ROPE_LEN - 1]));
                } else {
                    break;
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    println!(
        "{} {}",
        visited.iter().map(|k| k.0.abs()).max().unwrap(),
        visited.iter().map(|k| k.1.abs()).max().unwrap()
    );
    println!("{}", visited.len());
}
