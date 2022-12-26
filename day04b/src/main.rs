use common::parse;

pub fn main() {
    print!(
        "{} ",
        include_str!("../input.txt")
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(',').unwrap();
                let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
                (
                    parse::<u8>(a.as_bytes()),
                    parse::<u8>(b.as_bytes()),
                    parse::<u8>(c.as_bytes()),
                    parse::<u8>(d.as_bytes()),
                )
            })
            .filter(|(a, b, c, d)| b >= c && a <= d)
            .count()
    );
}
