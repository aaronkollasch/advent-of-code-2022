#[derive(Debug, Clone)]
enum Op {
    Square,
    Add(u64),
    Mul(u64),
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<u64>,
    pub test: u64,
    pub operation: Op,
    pub target1: usize,
    pub target2: usize,
    pub inspect_count: u64,
}

pub fn main() {
    let s = include_str!("../input.txt");
    let mut monkeys: Vec<Monkey> = Vec::<Monkey>::new();

    // parse input
    s.split("\n\n").enumerate().for_each(|(i_monkey, m)| {
        let mut monkey: Monkey = Monkey {
            items: Vec::with_capacity(32),
            test: 0,
            operation: Op::Square,
            target1: 0,
            target2: 0,
            inspect_count: 0,
        };
        for (i, l) in m.lines().enumerate() {
            match i {
                1 => {
                    monkey.items.extend(
                        l["  Starting items: ".len()..]
                            .split(", ")
                            .map(|w| w.parse::<u64>().expect(w)),
                    );
                }
                2 => {
                    let idx_start = "  Operation: new = old * ".len();
                    if l.chars().nth(idx_start - 2) == Some('+') {
                        monkey.operation = Op::Add(l[idx_start..].parse::<u64>().unwrap());
                    } else if &l[idx_start..] != "old" {
                        monkey.operation = Op::Mul(l[idx_start..].parse::<u64>().unwrap());
                    }
                }
                3 => {
                    monkey.test = l["  Test: divisible by ".len()..].parse::<u64>().unwrap();
                }
                4 => {
                    monkey.target1 = l["    If true: throw to monkey ".len()..]
                        .parse::<usize>()
                        .unwrap();
                    if i_monkey == monkey.target1 {
                        panic!("monkey {} targets itself!", i_monkey);
                    }
                }
                5 => {
                    monkey.target2 = l["    If false: throw to monkey ".len()..]
                        .parse::<usize>()
                        .unwrap();
                    if i_monkey == monkey.target2 {
                        panic!("monkey {} targets itself!", i_monkey);
                    }
                }
                _ => {}
            }
        }
        monkeys.push(monkey);
    });

    // lcm = product of all divisors as they are prime
    let lcm = monkeys.iter().map(|m| m.test).product::<u64>();
    #[cfg(debug_assertions)]
    eprintln!("lcm: {}", lcm);

    // simulate rounds
    for _round in 1..10001 {
        for i_monkey in 0..monkeys.len() {
            let ptr = monkeys.as_mut_ptr();
            let monkey = unsafe { &mut *ptr.add(i_monkey) };
            // #[cfg(debug_assertions)]
            // eprintln!("{} {} {:?}", i_monkey, monkey.inspect_count, monkey.items);
            monkey.items.iter_mut().for_each(|item| {
                monkey.inspect_count += 1;
                match monkey.operation {
                    Op::Add(y) => {
                        *item = item.wrapping_add(y);
                    }
                    Op::Mul(y) => {
                        *item = item.wrapping_mul(y) % lcm;
                    }
                    _ => {
                        *item = item.wrapping_mul(*item) % lcm;
                    }
                }
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
        .collect::<Vec<u64>>();
    counts.sort_unstable();
    print!("{} ", counts.iter().rev().take(2).product::<u64>());
}
