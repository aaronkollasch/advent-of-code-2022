use std::cmp::max;

type Row = u8;
type Rock = (u32, usize);

// rocks:
//
// ####
const ROCK_1: Rock = (0x0000001E, 1);
// .#.
// ###
// .#.
const ROCK_2: Rock = (0x00081C08, 3);
// ..#
// ..#
// ###
const ROCK_3: Rock = (0x0004041C, 3);
// #
// #
// #
// #
const ROCK_4: Rock = (0x10101010, 4);
// ##
// ##
const ROCK_5: Rock = (0x00001818, 2);

const ROCKS: [Rock; 5] = [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5];

const MAP_HEIGHT: usize = 4096;

#[inline]
fn shift_rock_left(rock: &mut Rock) {
    if rock.0 & 0x40404040 == 0 {
        rock.0 <<= 1;
    }
}

#[inline]
fn shift_rock_right(rock: &mut Rock) {
    if rock.0 & 0x01010101 == 0 {
        rock.0 >>= 1;
    }
}

struct Map {
    contents: [Row; MAP_HEIGHT],
    highest_rock: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            contents: [0; MAP_HEIGHT],
            highest_rock: 0,
        }
    }

    #[inline]
    pub fn get_row(&self, y: usize) -> Row {
        self.contents[y % MAP_HEIGHT]
    }

    #[inline]
    pub fn get_row_mut(&mut self, y: usize) -> &mut Row {
        &mut self.contents[y % MAP_HEIGHT]
    }

    #[allow(dead_code)]
    #[inline]
    pub fn set_row(&mut self, y: usize, val: Row) {
        self.contents[y % MAP_HEIGHT] = val;
    }

    #[inline]
    pub fn collides_with(&self, rock: Rock, rock_y: usize) -> bool {
        let mask = (rock_y..rock_y + rock.1)
            .rev()
            .map(|y| self.get_row(y))
            .fold(0, |acc, b| (acc << 8) | b as u32);

        rock.0 & mask != 0
    }

    #[inline]
    pub fn add_rock(&mut self, rock: Rock, rock_y: usize) {
        rock.0
            .to_le_bytes()
            .into_iter()
            .enumerate()
            .for_each(|(y, b_rock)| {
                let row = self.get_row_mut(rock_y + y);
                *row |= b_rock;
            });
        self.highest_rock = max(self.highest_rock, rock_y + rock.1);
    }
}

pub fn get_result() -> usize {
    let s = include_bytes!("../../inputs/day17.txt");
    let jet_len = s.len() - 1;
    let mut jet_i = 0;
    let mut map = Map::new();

    for i_rock in 0..2022 {
        let mut rock = ROCKS[i_rock % ROCKS.len()];
        let mut last_rock;
        let mut rock_y = map.highest_rock + 3;
        let mut last_y = rock_y;
        #[cfg(debug_assertions)]
        println!("{} ({}) {}", i_rock % ROCKS.len(), rock_y, map.highest_rock);
        while rock_y != usize::MAX && !map.collides_with(rock, rock_y) {
            #[cfg(debug_assertions)]
            println!("{} ({}) {}", i_rock % ROCKS.len(), rock_y, s[jet_i] as char);
            last_rock = rock;
            if s[jet_i] == b'<' {
                shift_rock_left(&mut rock);
            } else {
                shift_rock_right(&mut rock);
            }
            jet_i = (jet_i + 1) % jet_len;
            if map.collides_with(rock, rock_y) {
                rock = last_rock;
            }
            #[cfg(debug_assertions)]
            println!("{} ({}) v", i_rock % ROCKS.len(), rock_y);
            last_y = rock_y;
            rock_y = rock_y.wrapping_sub(1);
        }
        rock_y = last_y;
        map.add_rock(rock, rock_y);
        #[cfg(debug_assertions)]
        println!("{} ({}) {}", i_rock % ROCKS.len(), rock_y, map.highest_rock);
        #[cfg(debug_assertions)]
        {
            let highest_rock = map.highest_rock + 1;
            for y in (highest_rock.saturating_sub(16)..=highest_rock).rev() {
                println!(
                    "|{}| {}",
                    format!("{:07b}", map.get_row(y)).replace("0", " "),
                    y + 1
                );
            }
        }
    }
    return map.highest_rock;
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
        assert_eq!(result, 3239);
    }
}
