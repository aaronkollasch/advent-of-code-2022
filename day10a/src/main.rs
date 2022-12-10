#[inline]
fn calc_strength(cycle: usize, reg: [i32; 1]) -> i32 {
    (cycle as i32) * (reg[0] as i32)
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut cycle: usize = 0;
    let mut reg: [i32; 1] = [1];
    let mut strengths: [i32; 256] = [0; 256];

    s.split(|b| b == &b'\n').for_each(|l| match &l[0..4] {
        b"addx" => {
            cycle += 1;
            strengths[cycle - 1] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg[0], strengths[cycle - 1]);
            cycle += 1;
            strengths[cycle - 1] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg[0], strengths[cycle - 1]);
            reg[0] += atoi::atoi::<i32>(&l[5..]).unwrap();
        }
        b"noop" => {
            cycle += 1;
            strengths[cycle - 1] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg[0], strengths[cycle - 1]);
        }
        _ => {}
    });

    #[cfg(debug_assertions)]
    {
        eprintln!(
            "{:?}",
            strengths.iter().enumerate().collect::<Vec<(usize, &i32)>>()
        );
        eprintln!(
            "{:?}",
            strengths
                .iter()
                .enumerate()
                .skip(19)
                .step_by(40)
                .collect::<Vec<(usize, &i32)>>()
        );
    }
    println!("{}", strengths.iter().skip(19).step_by(40).sum::<i32>());
}
