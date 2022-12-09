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
    let (mut h, mut t, mut visited): ((i32, i32), (_, _), rustc_hash::FxHashSet<_>) =
        Default::default();
    visited.reserve(8192);
    visited.insert((0, 0));

    for (dir, dist) in cmds {
        for _i_step in 0..dist {
            h = (h.0 + dir.0, h.1 + dir.1);
            if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                t = (h.0 - dir.0, h.1 - dir.1);
                visited.insert(t);
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
