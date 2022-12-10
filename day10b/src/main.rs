pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut cycle: usize = 0;
    let mut reg: i8 = 1;
    let mut regs: [i8; 256] = [0; 256];

    s.split(|b| b == &b'\n').for_each(|l| match l[0] {
        b'a' => {
            cycle += 1;
            regs[cycle - 1] = reg;
            cycle += 1;
            regs[cycle - 1] = reg;
            let addx = match l[5] {
                b'-' => -l[6..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as i8),
                _ => l[5..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as i8),
            };
            reg += addx;
        }
        b'n' => {
            cycle += 1;
            regs[cycle - 1] = reg;
        }
        _ => {}
    });

    let mut crt = String::with_capacity(246);
    for cycle in 0..240 {
        let x = cycle % 40;
        if regs[cycle].abs_diff(x as i8) <= 1 {
            crt.push('@');
        } else {
            crt.push(' ');
        }
        if x == 39 {
            crt.push('\n');
        }
    }

    #[cfg(debug_assertions)]
    println!("{}", crt);
    println!("{}", crt.chars().take(40).collect::<String>());
}
