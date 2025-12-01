pub fn get_result() -> i16 {
    return include_bytes!("../../inputs/day02.txt")
        .split(|b| *b == b'\n')
        .filter(|l| l.len() == 3)
        .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16))
        .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
        .sum::<i16>();
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
        assert_eq!(result, 13526);
    }
}
