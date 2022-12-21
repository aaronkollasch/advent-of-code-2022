use rustc_hash::FxHashMap as HashMap;

type Number = isize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Div,
    Sub,
    Nop,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Monkey<'a> {
    ref1: &'a str,
    ref2: &'a str,
    op: Operation,
    val: Option<Number>,
}

pub fn main() {
    let s = include_str!("../input.txt");
    let mut monkeys = s
        .lines()
        .map(|l| {
            let name = &l[0..4];
            let mut ref1 = "";
            let mut ref2 = "";
            let mut val = None;
            let mut op = Operation::Nop;
            if l[6..].contains(' ') {
                ref1 = &l[6..10];
                ref2 = &l[13..17];
                op = match &l[11..12] {
                    "+" => Operation::Add,
                    "-" => Operation::Sub,
                    "*" => Operation::Mul,
                    "/" => Operation::Div,
                    _ => unreachable!(),
                }
            } else {
                val = Some(l[6..].parse::<Number>().unwrap());
            }
            (
                name,
                Monkey {
                    ref1,
                    ref2,
                    op,
                    val,
                },
            )
        })
        .collect::<HashMap<&str, Monkey>>();
    #[cfg(debug_assertions)]
    for monkey in monkeys.iter() {
        println!("{:?}", monkey);
    }
    let mut i = 0;
    let mut monkey_names = monkeys.iter().map(|(name, _)| *name).collect::<Vec<&str>>();
    while monkeys["root"].val.is_none() {
        #[cfg(debug_assertions)]
        println!("{}", i);
        if i > 100 {
            panic!("root still none");
        }
        monkey_names.retain(|name| {
            let monkey = monkeys[name];
            if monkey.op == Operation::Nop {
                return false;
            }
            #[cfg(debug_assertions)]
            println!("{} -> {}, {}", name, monkey.ref1, monkey.ref2);
            let (val1, val2) = (monkeys[monkey.ref1].val, monkeys[monkey.ref2].val);
            if let (None, Some(val1), Some(val2)) = (monkey.val, val1, val2) {
                let mut monkey = monkeys.get_mut(name).unwrap();
                match monkey.op {
                    Operation::Add => {
                        monkey.val = Some(val1 + val2);
                    }
                    Operation::Sub => {
                        monkey.val = Some(val1 - val2);
                    }
                    Operation::Mul => {
                        monkey.val = Some(val1 * val2);
                    }
                    Operation::Div => {
                        monkey.val = Some(val1 / val2);
                    }
                    _ => {}
                }
            }
            monkey.val.is_none()
        });
        i += 1;
    }
    #[cfg(debug_assertions)]
    {
        let (branch1, branch2) = (monkeys["root"].ref1, monkeys["root"].ref2);
        println!(
            "{}: {}, {}: {}",
            branch1,
            monkeys[branch1].val.unwrap(),
            branch2,
            monkeys[branch2].val.unwrap()
        );
    }
    print!("{} ", monkeys["root"].val.unwrap_or(Number::MAX));
}
