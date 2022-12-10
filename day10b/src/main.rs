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
            reg += atoi::atoi::<i8>(&l[5..]).unwrap();
        }
        b'n' => {
            cycle += 1;
            regs[cycle - 1] = reg;
        }
        _ => {}
    });

    let image = (0..240)
        .map(|c| {
            let x = c % 40;
            if regs[c].abs_diff(x as i8) <= 1 {
                '@'
            } else {
                ' '
            }
        })
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    #[cfg(debug_assertions)]
    println!("{}\n", image);
    println!("{}", image.chars().take(40).collect::<String>());
}
