pub fn main() {
    print!(
        "{} ",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .filter(|l| l.len() > 0)
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group| group[0]
                .iter()
                .find(|item| group[1].contains(item) && group[2].contains(item))
                .unwrap())
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
