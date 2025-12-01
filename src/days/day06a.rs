const WINDOW: usize = 4;

pub fn get_result() -> usize {
    return include_bytes!("../../inputs/day06.txt")
        .windows(WINDOW)
        .position(|b| !(0..WINDOW - 1).any(|i| (i + 1..WINDOW).any(|j| b[i] == b[j])))
        .unwrap()
        + WINDOW;
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
        assert_eq!(result, 1766);
    }
}
