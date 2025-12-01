use crate::common::parse_signed;

const WIDTH: usize = 40;
const LINES: usize = 6;
const CYCLES: usize = WIDTH * LINES;

pub fn get_result() -> String {
    let s = include_bytes!("../../inputs/day10.txt");
    let mut cycle: usize = 0;
    let mut reg: i8 = 1;
    let mut regs: [i8; CYCLES.next_power_of_two()] = [0; CYCLES.next_power_of_two()];

    s.split(|b| b == &b'\n').for_each(|l| match l[0] {
        b'a' => {
            cycle += 1;
            regs[cycle - 1] = reg;
            cycle += 1;
            regs[cycle - 1] = reg;
            let addx: i8 = parse_signed(&l[5..]);
            reg += addx;
        }
        b'n' => {
            cycle += 1;
            regs[cycle - 1] = reg;
        }
        _ => {}
    });

    let mut crt = String::with_capacity(CYCLES + LINES);
    for cycle in 0..CYCLES {
        let x = cycle % WIDTH;
        if regs[cycle].abs_diff(x as i8) <= 1 {
            crt.push('@');
        } else {
            crt.push(' ');
        }
    }

    return crt;
}

pub fn main() {
    let result = get_result();

    #[cfg(debug_assertions)]
    for line in 0..LINES {
        println!("{}", &result[line * WIDTH..(line + 1) * WIDTH]);
    }

    #[cfg(not(debug_assertions))]
    print!("{} ", get_result());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        let result = get_result();
        assert_eq!(result, "@@@@ @  @ @@@  @@@@ @@@    @@  @@  @    @    @  @ @  @    @ @  @    @ @  @ @    @@@  @@@@ @  @   @  @  @    @ @    @    @    @  @ @@@   @   @@@     @ @ @@ @    @    @  @ @    @    @    @  @ @  @ @    @@@@ @  @ @    @@@@ @     @@   @@@ @@@@ ");
    }
}
