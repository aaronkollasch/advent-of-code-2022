#[inline]
fn calc_strength(cycle: usize, reg: i8) -> i32 {
    (cycle as i32) * (reg as i32)
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut cycle: usize = 0;
    let mut reg: i8 = 1;
    let mut strengths: [i32; 256] = [0; 256];

    s.split(|b| b == &b'\n').for_each(|l| match l[0] {
        b'a' => {
            cycle += 1;
            strengths[cycle] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg, strengths[cycle]);
            cycle += 1;
            strengths[cycle] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg, strengths[cycle]);
            reg[0] += atoi::atoi::<i32>(&l[5..]).unwrap();
        }
        b'n' => {
            cycle += 1;
            strengths[cycle] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg, strengths[cycle]);
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
                .skip(20)
                .step_by(40)
                .collect::<Vec<(usize, &i32)>>()
        );
    }
    println!("{}", strengths.iter().skip(20).step_by(40).sum::<i32>());
}
