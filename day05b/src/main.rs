use itertools::Itertools;
use std::collections::LinkedList;

pub fn main() {
    let d = include_bytes!("../input.txt");
    let (stacks, instructions) = d.split_at(d.windows(2).position(|b| b == b"\n\n").unwrap() + 2);

    // load initial stack state
    let mut stack_lines = stacks.split(|b| *b == b'\n').rev().skip(2);
    let num_stacks = stack_lines.next().unwrap().len() / 4 + 1;
    let mut stacks: Vec<LinkedList<u8>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(LinkedList::<u8>::new());
    }
    stack_lines.for_each(|l| {
        for i in 0..num_stacks {
            let c = l[1 + i * 4];
            if c != b' ' {
                stacks[i].push_back(c);
            }
        }
    });

    // execute instructions
    instructions
        .split(|b| *b == b'\n')
        .filter(|l| l.len() > 0)
        .for_each(|l| {
            let (n, from, to): (usize, _, _) = l
                .split(|b| b == &b' ')
                .skip(1)
                .step_by(2)
                .map(|n| atoi::atoi(n).unwrap())
                .collect_tuple()
                .unwrap();
            let split_index = stacks[from - 1].len() - n;
            let mut stack_part = stacks.get_mut(from - 1).unwrap().split_off(split_index);
            stacks.get_mut(to - 1).unwrap().append(&mut stack_part);
        });

    // print last stacks
    println!(
        "{}",
        stacks
            .iter()
            .map(|c| *c.back().unwrap() as char)
            .collect::<String>()
    );
}
