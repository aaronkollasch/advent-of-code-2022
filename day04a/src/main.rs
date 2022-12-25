#[inline]
fn parse(b: &[u8]) -> u8 {
    b.iter().fold(0, |acc, x| acc * 10 + (x - b'0'))
}

pub fn main() {
    print!(
        "{} ",
        include_str!("../input.txt")
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(',').unwrap();
                let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
                (
                    parse(a.as_bytes()),
                    parse(b.as_bytes()),
                    parse(c.as_bytes()),
                    parse(d.as_bytes()),
                )
            })
            .filter(|(a, b, c, d)| (a <= c && b >= d) || (a >= c && b <= d))
            .count()
    );
}
