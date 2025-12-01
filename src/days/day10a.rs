use crate::common::parse_signed;

const WIDTH: usize = 40;
const LINES: usize = 6;
const CYCLES: usize = WIDTH * LINES;

#[inline]
fn calc_strength(cycle: usize, reg: i8) -> i32 {
    (cycle as i32) * (reg as i32)
}

pub fn get_result() -> i32 {
    let s = include_bytes!("../../inputs/day10.txt");
    let mut cycle: usize = 0;
    let mut reg: i8 = 1;
    let mut strengths: [i32; CYCLES.next_power_of_two()] = [0; CYCLES.next_power_of_two()];

    s.split(|b| b == &b'\n').for_each(|l| match l[0] {
        b'a' => {
            let addx: i8 = parse_signed(&l[5..]);
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
    return strengths.iter().skip(20).step_by(WIDTH).sum::<i32>();
}

pub fn main() {
    print!("{} ", get_result());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        let result = get_result();
        assert_eq!(result, 12520);
    }
}
