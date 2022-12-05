pub fn main() {
    let (stacks, instructions) = include_str!("../target.txt").split_once("\n\n").unwrap();

    // load initial stack state
    let mut stack_lines = stacks.lines().rev();
    let stack_indices = stack_lines.next().unwrap();
    let num_stacks = stack_indices.len() / 4 + 1;
    let mut stacks: Vec<Vec<u8>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::<u8>::new());
    }
    stack_lines.for_each(|l| {
        for i in 0..num_stacks {
            let c = l.as_bytes()[1 + i * 4];
            if c != b' ' {
                stacks[i].push(c);
            }
        }
    });

    // execute instructions
    instructions.lines().enumerate().for_each(|(i_line, l)| {
        let mut l_words = l.split(' ');
        let n = l_words.nth(1).unwrap().parse::<usize>().unwrap();
        let from = l_words.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = l_words.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        for i in 0..n {
            let c = stacks
                .get_mut(from)
                .expect(format!("{}, {}: get stack {}: {}", i_line, i, from, num_stacks).as_str())
                .pop()
                .expect(
                    format!(
                        "{}, {}: pop {} from {}: {}",
                        i_line,
                        i,
                        n,
                        from,
                        stacks[from].len()
                    )
                    .as_str(),
                );
            stacks
                .get_mut(to)
                .expect(format!("{}, {}: get stack {}: {}", i_line, i, to, num_stacks).as_str())
                .push(c);
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
