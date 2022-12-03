use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut top_elves: BinaryHeap<Reverse<u32>> = BinaryHeap::with_capacity(4);
    top_elves.push(Reverse::<u32>(0));
    top_elves.push(Reverse::<u32>(0));
    top_elves.push(Reverse::<u32>(0));

    include_str!("../input.txt").split("\n\n").for_each(|elf| {
        top_elves.push(Reverse::<u32>(
            elf.lines()
                .map(|c| c.parse::<u32>().unwrap_or(0))
                .sum::<u32>(),
        ));
        top_elves.pop();
    });
    println!("{}", top_elves.iter().map(|e| e.0).sum::<u32>());
}
