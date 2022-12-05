use std::collections::LinkedList;

pub fn main() {
    let (stacks, instructions) = include_str!("../target.txt").split_once("\n\n").unwrap();

    // load initial stack state
    let mut stack_lines = stacks.lines().rev();
    let stack_indices = stack_lines.next().unwrap();
    let num_stacks = stack_indices.len() / 4 + 1;
    let mut stacks: Vec<LinkedList<u8>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(LinkedList::<u8>::new());
    }
    stack_lines.for_each(|l| {
        for i in 0..num_stacks {
            let c = l.as_bytes()[1 + i * 4];
            if c != b' ' {
                stacks[i].push_back(c);
            }
        }
    });

    // execute instructions
    instructions.lines().enumerate().for_each(|(i_line, l)| {
        let mut l_words = l.split(' ');
        let n = l_words.nth(1).unwrap().parse::<usize>().unwrap();
        let from = l_words.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = l_words.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let split_index = stacks[from].len() - n;
        let mut stack_part = stacks
            .get_mut(from)
            .expect(format!("{}: get stack {}: {}", i_line, from, num_stacks).as_str())
            .split_off(split_index);
        stacks
            .get_mut(to)
            .expect(format!("{}: get stack {}: {}", i_line, to, num_stacks).as_str())
            .append(&mut stack_part);
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
