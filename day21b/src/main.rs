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
            if l.len() > 10 {
                ref1 = &l[6..10];
                ref2 = &l[13..17];
                op = match (&l[11..12], name) {
                    (_, "root") => Operation::Sub,
                    ("+", _) => Operation::Add,
                    ("-", _) => Operation::Sub,
                    ("*", _) => Operation::Mul,
                    ("/", _) => Operation::Div,
                    _ => unreachable!(),
                }
            } else {
                val = match name {
                    "humn" => None,
                    _ => Some(l[6..].parse::<Number>().unwrap()),
                };
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
    let mut i = 0;
    let mut monkey_names = monkeys.iter().map(|(name, _)| *name).collect::<Vec<&str>>();
    let (branch1, branch2) = (monkeys["root"].ref1, monkeys["root"].ref2);
    while monkeys[branch2].val.is_none() {
        #[cfg(debug_assertions)]
        println!("{}", i);
        if i > 100 {
            panic!("{} still none", branch2);
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
    let target_val = monkeys[branch2].val.unwrap_or(Number::MAX);
    #[cfg(debug_assertions)]
    println!("target value: {} ", target_val);
    let monkeys_to_reset = monkey_names.clone();
    let mut low = 0;
    let mut high = 4037901067830;
    let mut humn_val: Number;
    loop {
        let mid = low + (high - low) / 2;
        #[cfg(debug_assertions)]
        println!("{} {} {}", low, mid, high);
        humn_val = mid;
        for name in monkeys_to_reset.iter() {
            monkeys.get_mut(name).unwrap().val = None;
        }
        monkeys.get_mut("humn").unwrap().val = Some(humn_val);
        let mut monkey_names = monkeys_to_reset.clone();
        i = 0;
        while monkeys[branch1].val.is_none() {
            if i > 100 {
                #[cfg(debug_assertions)]
                panic!("{} still none", branch1);
                break;
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
        let diff = monkeys[branch1].val.unwrap_or(Number::MAX) - target_val;
        #[cfg(debug_assertions)]
        println!("{} {}", humn_val, diff);
        match diff.cmp(&0) {
            std::cmp::Ordering::Greater => {
                low = mid;
            }
            std::cmp::Ordering::Less => {
                high = mid;
            }
            std::cmp::Ordering::Equal => {
                #[cfg(debug_assertions)]
                println!("humn val: {}", humn_val);
                break;
            }
        }
    }
    loop {
        let next_humn_val = humn_val - 1;
        #[cfg(debug_assertions)]
        println!("{}", next_humn_val);
        for name in monkeys_to_reset.iter() {
            monkeys.get_mut(name).unwrap().val = None;
        }
        monkeys.get_mut("humn").unwrap().val = Some(next_humn_val);
        let mut monkey_names = monkeys_to_reset.clone();
        i = 0;
        while monkeys[branch1].val.is_none() {
            if i > 100 {
                panic!("{} still none", branch1);
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
        let diff = monkeys[branch1].val.unwrap_or(Number::MAX) - target_val;
        #[cfg(debug_assertions)]
        println!("{} {}", humn_val, diff);
        match diff.cmp(&0) {
            std::cmp::Ordering::Equal => {
                humn_val = next_humn_val;
            }
            _ => {
                break;
            }
        }
    }
    #[cfg(debug_assertions)]
    println!(
        "{}: {}, {}: {}",
        branch1,
        monkeys[branch1].val.unwrap(),
        branch2,
        monkeys[branch2].val.unwrap()
    );
    print!("{} ", humn_val);
}
