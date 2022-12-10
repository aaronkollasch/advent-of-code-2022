const WIDTH: usize = 40;
const LINES: usize = 6;
const CYCLES: usize = WIDTH * LINES;

#[inline]
fn calc_strength(cycle: usize, reg: i8) -> i32 {
    (cycle as i32) * (reg as i32)
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut cycle: usize = 0;
    let mut reg: i8 = 1;
    let mut strengths: [i32; CYCLES.next_power_of_two()] = [0; CYCLES.next_power_of_two()];

    s.split(|b| b == &b'\n').for_each(|l| match l[0] {
        b'a' => {
            let addx = match l[5] {
                b'-' => -l[6..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as i8),
                _ => l[5..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as i8),
            };
            #[cfg(debug_assertions)]
            eprintln!("addx {}", addx);
            cycle += 1;
            strengths[cycle] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg, strengths[cycle]);
            cycle += 1;
            strengths[cycle] = calc_strength(cycle, reg);
            #[cfg(debug_assertions)]
            eprintln!("{} {} {}", cycle, reg, strengths[cycle]);
            reg += addx;
        }
        b'n' => {
            #[cfg(debug_assertions)]
            eprintln!("noop");
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
                .step_by(WIDTH)
                .collect::<Vec<(usize, &i32)>>()
        );
    }
    print!("{} ", strengths.iter().skip(20).step_by(WIDTH).sum::<i32>());
}
