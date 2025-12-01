use primitive_types::U256;
use u256_literal::u256;

type Number = usize;
type Pos = (Number, Number);

#[allow(dead_code)]
const MAX_COLS: usize = 256;
const MAX_ROWS: usize = 32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Grid {
    vals: [U256; MAX_ROWS],
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            vals: [u256!(0); MAX_ROWS],
            width,
            height,
        }
    }

    fn get_val(&self, pos: Pos) -> bool {
        (self.vals[pos.1] >> pos.0) & u256!(1) > u256!(0)
    }

    fn insert_val(&mut self, pos: Pos) {
        self.vals[pos.1] |= u256!(1) << pos.0;
    }

    fn blow_up(&mut self) {
        self.vals[..self.height].rotate_left(1);
    }

    fn blow_down(&mut self) {
        self.vals[..self.height].rotate_right(1);
    }

    fn blow_left(&mut self) {
        for y in 0..self.height {
            self.vals[y] = (self.vals[y] >> 1) | ((self.vals[y] & u256!(1)) << (self.width - 1));
        }
    }

    fn blow_right(&mut self) {
        for y in 0..self.height {
            self.vals[y] = (self.vals[y] << 1) | (self.vals[y] >> (self.width - 1));
        }
    }

    fn expand_one_and_filter(&mut self, obstacles: [[U256; MAX_ROWS]; 4]) {
        let mut above: U256 = u256!(0);
        for y in 0..self.height {
            let row = self.vals[y];
            self.vals[y] |= above | (row >> 1) | (row << 1 & ((u256!(1) << self.width) - 1));
            if y + 1 < self.height {
                self.vals[y] |= self.vals[y + 1];
            }
            above = row;

            let obstacle = obstacles[0][y] | obstacles[1][y] | obstacles[2][y] | obstacles[3][y];
            self.vals[y] &= !obstacle;
        }
    }
}

#[cfg(debug_assertions)]
fn debug_print(
    t: usize,
    h: usize,
    w: usize,
    hurricanes_up: Grid,
    hurricanes_down: Grid,
    hurricanes_left: Grid,
    hurricanes_right: Grid,
    elves: Grid,
) {
    println!("\ntime: {}", t);
    for y in 0..h {
        for x in 0..w {
            let vars = [
                hurricanes_right.get_val((x, y)),
                hurricanes_left.get_val((x, y)),
                hurricanes_up.get_val((x, y)),
                hurricanes_down.get_val((x, y)),
                elves.get_val((x, y)),
            ];
            match vars[0..4].iter().map(|b| *b as u8).sum::<u8>() {
                s if s > 1 => { print!("{}", s); }
                s if s == 1 && vars[0] => { print!(">"); }
                s if s == 1 && vars[1] => { print!("<"); }
                s if s == 1 && vars[2] => { print!("^"); }
                s if s == 1 && vars[3] => { print!("v"); }
                _ if vars[4] => { print!("*"); }
                _ => { print!(" "); }
            }
        }
        println!();
    }
    println!();
}

pub fn get_result() -> usize {
    let s = include_bytes!("../../inputs/day24.txt");
    let w = s.iter().position(|b| *b == b'\n').unwrap() - 2;
    let h = s.len() / (w + 3) - 2;
    let mut hurricanes_up = Grid::new(w, h);
    let mut hurricanes_down = Grid::new(w, h);
    let mut hurricanes_left = Grid::new(w, h);
    let mut hurricanes_right = Grid::new(w, h);
    let mut elves = Grid::new(w, h);

    #[cfg(debug_assertions)]
    println!("w: {}, h: {}", w, h);
    s.split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .skip(1)
        .take(h)
        .enumerate()
        .for_each(|(y, l)| {
            l[1..]
                .iter()
                .take(w)
                .enumerate()
                .for_each(|(x, b)| match b {
                    b'>' => {
                        hurricanes_right.insert_val((x, y));
                    }
                    b'<' => {
                        hurricanes_left.insert_val((x, y));
                    }
                    b'v' => {
                        hurricanes_down.insert_val((x, y));
                    }
                    b'^' => {
                        hurricanes_up.insert_val((x, y));
                    }
                    _ => {}
                })
        });
    let mut t = 0;
    #[cfg(debug_assertions)]
    debug_print(
        t,
        h,
        w,
        hurricanes_up,
        hurricanes_down,
        hurricanes_left,
        hurricanes_right,
        elves,
    );
    while !elves.get_val((w - 1, h - 1)) {
        hurricanes_left.blow_left();
        hurricanes_right.blow_right();
        hurricanes_up.blow_up();
        hurricanes_down.blow_down();
        elves.expand_one_and_filter([
            hurricanes_up.vals,
            hurricanes_down.vals,
            hurricanes_left.vals,
            hurricanes_right.vals,
        ]);
        if !hurricanes_right.get_val((0, 0))
            && !hurricanes_left.get_val((0, 0))
            && !hurricanes_up.get_val((0, 0))
            && !hurricanes_down.get_val((0, 0))
        {
            elves.insert_val((0, 0));
        }
        t += 1;
        #[cfg(debug_assertions)]
        debug_print(
            t,
            h,
            w,
            hurricanes_up,
            hurricanes_down,
            hurricanes_left,
            hurricanes_right,
            elves,
        );
    }
    t += 1;
    return t;
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
        assert_eq!(result, 343);
    }
}
