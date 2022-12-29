const WINDOW: usize = 4;

pub fn main() {
    print!(
        "{} ",
        include_bytes!("../input.txt")
            .windows(WINDOW)
            .position(|b| !(0..WINDOW - 1).any(|i| (i + 1..WINDOW).any(|j| b[i] == b[j])))
            .unwrap()
            + WINDOW,
    );
}
