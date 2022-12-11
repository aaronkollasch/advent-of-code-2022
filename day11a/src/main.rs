struct Monkey {
    pub items: Vec<u64>,
    pub test: u64,
    pub operation: (u8, u64),
    pub target1: usize,
    pub target2: usize,
    pub inspect_count: u32,
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut monkeys: Vec<Monkey> = Vec::<Monkey>::new();

    // parse input
    for (i_line, l) in s.split(|b| *b == b'\n').filter(|l| l.len() > 0).enumerate() {
        let (i_monkey, i_line) = (i_line / 6, i_line % 6);
        if i_line == 0 {
            monkeys.push(Monkey {
                items: Vec::with_capacity(32),
                test: 0,
                operation: (0, 0),
                target1: 0,
                target2: 0,
                inspect_count: 0,
            });
        }
        let mut monkey = monkeys
            .get_mut(i_monkey)
            .unwrap_or_else(|| panic!("{}", i_monkey));
        match i_line {
            1 => {
                monkey.items.extend(
                    l[17..]
                        .split(|b| *b == b',')
                        .map(|w| w[1..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u64)),
                );
            }
            2 => {
                if l[23] == b'+' {
                    monkey.operation = (
                        0,
                        l[25..]
                            .iter()
                            .fold(0, |acc, x| acc * 10 + (x - b'0') as u64),
                    );
                } else if l[25] == b'o' {
                    monkey.operation = (2, 0);
                } else {
                    monkey.operation = (
                        1,
                        l[25..]
                            .iter()
                            .fold(0, |acc, x| acc * 10 + (x - b'0') as u64),
                    );
                }
            }
            3 => {
                monkey.test = l[21..]
                    .iter()
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as u64);
            }
            4 => {
                monkey.target1 = l[29..]
                    .iter()
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as usize);
                if i_monkey == monkey.target1 {
                    panic!("monkey {} targets itself!", i_monkey);
                }
            }
            5 => {
                monkey.target2 = l[30..]
                    .iter()
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as usize);
                if i_monkey == monkey.target2 {
                    panic!("monkey {} targets itself!", i_monkey);
                }
            }
            _ => {}
        }
    }
    #[cfg(debug_assertions)]
    eprintln!("monkeys: {}", monkeys.len());

    // simulate rounds
    for _round in 1..21 {
        for i_monkey in 0..monkeys.len() {
            let ptr = monkeys.as_mut_ptr();
            let monkey = unsafe { &mut *ptr.add(i_monkey) };
            #[cfg(debug_assertions)]
            eprintln!("{} {} {:?}", i_monkey, monkey.inspect_count, monkey.items);
            monkey.items.iter_mut().for_each(|item| {
                monkey.inspect_count += 1;
                match monkey.operation.0 {
                    0 => {
                        *item += monkey.operation.1;
                    }
                    1 => {
                        *item *= monkey.operation.1;
                    }
                    _ => {
                        *item *= *item;
                    }
                }
                *item /= 3;
                if *item % monkey.test == 0 {
                    monkeys[monkey.target1].items.push(*item);
                } else {
                    monkeys[monkey.target2].items.push(*item);
                }
            });
            monkey.items.clear();
        }
    }

    let mut counts = monkeys
        .iter()
        .map(|m| m.inspect_count)
        .collect::<Vec<u32>>();
    counts.sort_unstable();
    print!("{} ", counts.iter().rev().take(2).product::<u32>());
}
