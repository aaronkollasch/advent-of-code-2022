use crate::common::parse;

pub fn get_result() -> usize {
    return include_str!("../../inputs/day04.txt")
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
        .count();
}

pub fn main() {
    print!("{} ", get_result());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        let result = get_result();
        assert_eq!(result, 811);
    }
}
