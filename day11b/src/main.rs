struct Monkey {
    pub items: Vec<u64>,
    pub test: u64,
    pub operation: (u8, u64),
    pub target1: usize,
    pub target2: usize,
    pub inspect_count: u64,
}

pub fn main() {
    let s = include_str!("../input.txt");
    let mut monkeys: Vec<Monkey> = Vec::<Monkey>::new();

    s.split("\n\n").for_each(|m| {
        let mut monkey: Monkey = Monkey {
            items: vec![],
            test: 0,
            operation: (0, 0),
            target1: 0,
            target2: 0,
            inspect_count: 0,
        };
        for (i, l) in m.lines().enumerate() {
            match i {
                1 => {
                    monkey
                        .items
                        .extend(l.split_once(": ").unwrap().1.split(", ").map(|w| w.parse::<u64>().expect(w)));
                }
                2 => {
                    if l.chars().nth(23) == Some('+') {
                        monkey.operation = (0, l[25..].parse::<u64>().unwrap());
                    } else if &l[25..] == "old" {
                        monkey.operation = (2, 0);
                    } else {
                        monkey.operation = (1, l[25..].parse::<u64>().unwrap());
                    }
                }
                3 => {
                    monkey.test = l[21..].parse::<u64>().unwrap();
                }
                4 => {
                    monkey.target1 = l[29..].parse::<usize>().unwrap();
                }
                5 => {
                    monkey.target2 = l[30..].parse::<usize>().unwrap();
                }
                _ => {}
            }
        }
        monkeys.push(monkey);
    });

    let gcd = monkeys.iter().map(|m| m.test).product::<u64>();
    #[cfg(debug_assertions)]
    eprintln!("gcd: {}", gcd);

    for _round in 1..10001 {
        for i_monkey in 0..monkeys.len() {
            let mut passes: Vec<(usize, u64)> = vec![];
            let mut monkey = monkeys.get_mut(i_monkey).unwrap();
            // #[cfg(debug_assertions)]
            // eprintln!("{} {} {:?}", i_monkey, monkey.inspect_count, monkey.items);
            monkey.items.retain_mut(|item| {
                monkey.inspect_count += 1;
                match monkey.operation.0 {
                    0 => { *item = item.wrapping_add(monkey.operation.1); }
                    1 => { *item = item.wrapping_mul(monkey.operation.1); }
                    _ => { *item = item.wrapping_mul(*item); }
                }
                *item %= gcd;
                if *item % monkey.test == 0 {
                    passes.push((monkey.target1, *item));
                } else {
                    passes.push((monkey.target2, *item));
                }
                false
            });
            for (target, item) in passes {
                monkeys[target].items.push(item);
            }
        }
    }
    let mut counts = monkeys.iter().map(|m| m.inspect_count).collect::<Vec::<u64>>();
    counts.sort_unstable();
    println!("{}", counts.iter().rev().take(2).product::<u64>());
}
