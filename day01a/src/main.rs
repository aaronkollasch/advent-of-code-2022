fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|elf| elf.lines().map(|c| c.parse::<u32>().unwrap_or(0)).sum::<u32>())
            .max()
            .unwrap_or(0)
    );
}
