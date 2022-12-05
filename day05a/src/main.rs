use itertools::Itertools;

pub fn main() {
    let d = include_bytes!("../input.txt");
    let (stacks, instructions) = d.split_at(d.windows(2).position(|b| b == b"\n\n").unwrap() + 2);

    // load initial stack state
    let mut stack_lines = stacks.split(|b| *b == b'\n').rev().skip(2);
    let num_stacks = stack_lines.next().unwrap().len() / 4 + 1;
    let mut stacks: Vec<Vec<u8>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::<u8>::new());
    }
    stack_lines.for_each(|l| {
        for i in 0..num_stacks {
            let c = l[1 + i * 4];
            if c != b' ' {
                stacks[i].push(c);
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
            for _i in 0..n {
                let c = stacks.get_mut(from-1).unwrap().pop().unwrap();
                stacks.get_mut(to-1).unwrap().push(c);
            }
        });

    // print last stacks
    println!(
        "{}",
        stacks
            .iter()
            .map(|c| *c.last().unwrap() as char)
            .collect::<String>()
    );
}
