use itertools::Itertools;

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
    pub inspect_count: u32,
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut monkeys: Vec<Monkey> = Vec::<Monkey>::new();

    // parse input
    for (i_line, l) in s.split(|b| *b == b'\n').filter(|l| !l.is_empty()).enumerate() {
        let (i_monkey, i_line) = (i_line / 6, i_line % 6);
        if i_line == 0 {
            monkeys.push(Monkey {
                items: Vec::with_capacity(32),
                test: 0,
                operation: Op::Square,
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
                    l["  Starting items:".len()..]
                        .split(|b| *b == b',')
                        .map(|w| w[1..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u64)),
                );
            }
            2 => {
                let idx_start = "  Operation: new = old * ".len();
                if l[idx_start - 2] == b'+' {
                    monkey.operation = Op::Add(
                        l[idx_start..]
                            .iter()
                            .fold(0, |acc, x| acc * 10 + (x - b'0') as u64),
                    );
                } else if l[idx_start] != b'o' {
                    monkey.operation = Op::Mul(
                        l[idx_start..]
                            .iter()
                            .fold(0, |acc, x| acc * 10 + (x - b'0') as u64),
                    );
                }
            }
            3 => {
                monkey.test = l["  Test: divisible by ".len()..]
                    .iter()
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as u64);
            }
            4 => {
                monkey.target1 = l["    If true: throw to monkey ".len()..]
                    .iter()
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as usize);
                if i_monkey == monkey.target1 {
                    panic!("monkey {} targets itself!", i_monkey);
                }
            }
            5 => {
                monkey.target2 = l["    If false: throw to monkey ".len()..]
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
            monkey.inspect_count += monkey.items.len() as u32;
            for item in monkey.items.iter_mut() {
                *item = match monkey.operation {
                    Op::Add(y) => *item + y,
                    Op::Mul(y) => *item * y,
                    Op::Square => *item * *item,
                } / 3;
                if *item % monkey.test == 0 {
                    monkeys[monkey.target1].items.push(*item);
                } else {
                    monkeys[monkey.target2].items.push(*item);
                }
            }
            monkey.items.clear();
        }
    }

    print!(
        "{} ",
        monkeys
            .iter()
            .map(|m| m.inspect_count)
            .sorted_unstable_by(|a, b| b.cmp(a))
            .take(2)
            .product::<u32>()
    );
}
