use std::collections::HashMap;

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
struct Monkey <'a> {
    ref1: &'a str,
    ref2: &'a str,
    op: Operation,
    val: Option<Number>,
}

pub fn main() {
    // let s = include_bytes!("../input.txt");
    // s.split(|b| *b == b'\n').filter(|l| !l.is_empty()).for_each(|l| {
    // });
    // print!("{} ", s[0]);
    let s = include_str!("../input.txt");
    let mut monkeys = s.lines().map(|l| {
        let name = &l[0..4];
        let mut ref1 = "";
        let mut ref2 = "";
        let mut val = None;
        let mut op = Operation::Nop;
        if l.len() > 10 {
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
        (name, Monkey { ref1, ref2, op, val })
    }).collect::<HashMap<&str, Monkey>>();
    #[cfg(debug_assertions)]
    for monkey in monkeys.iter() {
        println!("{:?}", monkey);
    }
    let mut i = 0;
    let monkey_names = monkeys.iter().map(|(name, _)| *name).collect::<Vec<&str>>();
    while monkeys["root"].val.is_none() {
        #[cfg(debug_assertions)]
        println!("{}", i);
        if i > 100 {
            println!("root still none");
            break;
        }
        for name in monkey_names.iter() {
            let monkey = monkeys[name];
            if monkey.op == Operation::Nop {
                continue;
            }
            #[cfg(debug_assertions)]
            println!("{} -> {}, {}", name, monkey.ref1, monkey.ref2);
            let (val1, val2) = (monkeys[monkey.ref1].val, monkeys[monkey.ref2].val);
            if monkey.val.is_none() && val1.is_some() && val2.is_some() {
                let mut monkey = monkeys.get_mut(name).unwrap();
                match monkey.op {
                    Operation::Add => { monkey.val = Some(val1.unwrap() + val2.unwrap()); }
                    Operation::Sub => { monkey.val = Some(val1.unwrap() - val2.unwrap()); }
                    Operation::Mul => { monkey.val = Some(val1.unwrap() * val2.unwrap()); }
                    Operation::Div => { monkey.val = Some(val1.unwrap() / val2.unwrap()); }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    print!("{} ", monkeys["root"].val.unwrap_or(Number::MAX));
}
