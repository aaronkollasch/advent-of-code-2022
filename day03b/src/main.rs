use itertools::Itertools;

pub fn main() {
    print!(
        "{} ",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .filter(|l| l.len() > 0)
            .chunks(3)
            .into_iter()
            .map(|mut group| {
                let (a, b, c) = (
                    group.next().unwrap(),
                    group.next().unwrap(),
                    group.next().unwrap(),
                );
                a.iter()
                    .find(|item| b.contains(*item) && c.contains(*item))
                    .unwrap()
            })
            .map(|b| {
                if *b >= b'a' {
                    (b - b'a') as u16 + 1
                } else {
                    (b - b'A') as u16 + 27
                }
            })
            .sum::<u16>()
    );
}
