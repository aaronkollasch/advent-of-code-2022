pub fn main() {
    print!(
        "{} ",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .filter(|l| l.len() == 3)
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
            .sum::<i16>()
    );
}
