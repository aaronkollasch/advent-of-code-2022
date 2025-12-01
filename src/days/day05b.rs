use crate::common::parse;
use itertools::Itertools;
use std::collections::LinkedList;

pub fn get_result() -> String {
    let d = include_bytes!("../../inputs/day05.txt");
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
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(i_line, l)| {
            let (n, from, to): (usize, _, _) = l
                .split(|b| b == &b' ')
                .skip(1)
                .step_by(2)
                .map(parse::<usize>)
                .collect_tuple()
                .unwrap();
            let split_index = stacks[from - 1].len() - n;
            let mut stack_part = stacks
                .get_mut(from - 1)
                .unwrap_or_else(|| panic!("{}: get stack {}: {}", i_line, from - 1, num_stacks))
                .split_off(split_index);
            stacks
                .get_mut(to - 1)
                .unwrap_or_else(|| panic!("{}: get stack {}: {}", i_line, to - 1, num_stacks))
                .append(&mut stack_part);
        });

    // return last stacks
    return stacks
        .iter()
        .map(|c| *c.back().unwrap() as char)
        .collect::<String>();
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
        assert_eq!(result, "NBTVTJNFJ");
    }
}
