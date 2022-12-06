const WINDOW: usize = 4;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .windows(WINDOW)
            .enumerate()
            // https://stackoverflow.com/a/46766782/653173
            .find(|(_, s)| !(1..s.len()).any(|i| s[i..].contains(&s[i - 1])))
            .unwrap()
            .0
            + WINDOW
    );
}
