// use std::collections::HashSet;

pub fn main() {
    print!(
        "{} ",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .filter(|l| l.len() > 0)
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| {
                // let mut a_set: HashSet<u8> = HashSet::with_capacity(a.len());
                // for i in 0..a.len() {
                //     a_set.insert(a[i]);
                // };
                b.iter()
                    .filter(|b| a.contains(b))
                    .map(|b| {
                        if *b >= b'a' {
                            (b - b'a') as u16 + 1
                        } else {
                            (b - b'A') as u16 + 27
                        }
                    })
                    .next()
                    .unwrap()
            })
            .sum::<u16>()
    );
}
