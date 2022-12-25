use std::cmp::max;

pub fn main() {
    let mut elf_total = 0u32;
    let mut max_elf = 0u32;
    include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .for_each(|l| {
            if l.is_empty() {
                max_elf = max(max_elf, elf_total);
                elf_total = 0;
            } else {
                elf_total += l.iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u32);
            }
        });
    print!("{} ", max_elf);
}
