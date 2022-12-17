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

const MAP_HEIGHT: usize = 128;

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
    map_height: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            contents: [0; MAP_HEIGHT],
            highest_rock: 0,
            map_height: 20,
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

    #[inline]
    pub fn set_row(&mut self, y: usize, val: Row) {
        self.contents[y % MAP_HEIGHT] = val;
    }

    #[inline]
    pub fn collides_with(&self, rock: Rock, rock_y: usize) -> bool {
        if rock_y.saturating_add(rock.1) >= self.map_height { return true; }
        let mask = (rock_y..rock_y + rock.1)
            .rev()
            .map(|y| self.get_row(y))
            .fold(0, |acc, b| (acc << 8) | b as u32);

        rock.0 & mask != 0
    }

    pub fn add_rock(&mut self, rock: Rock, rock_y: usize) {
        rock.0.to_le_bytes().into_iter()
            .enumerate()
            .for_each(|(y, b_rock)| {
                let row = self.get_row_mut(rock_y + y);
                *row |= b_rock;
            });
        for y in self.highest_rock..self.highest_rock + 4 {
            if self.get_row(y) > 0 {
                self.highest_rock = y + 1;
            }
        }
        for y in self.map_height..self.highest_rock + 20 {
            self.set_row(y, 0);
        }
        self.map_height = self.highest_rock + 20;
    }
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let jet_len = s.len() - 1;
    let mut jet_i = 0;
    let mut map = Map::new();

    for i_rock in 0..2022 {
        let rock_i = i_rock % ROCKS.len();
        let mut rock = ROCKS[rock_i];
        let mut last_rock;
        let mut rock_y = map.highest_rock + 3;
        let mut last_y = rock_y;
        #[cfg(debug_assertions)]
        println!(
            "{} ({}) {}",
            rock_i, rock_y, map.highest_rock
        );
        while !map.collides_with(rock, rock_y) {
            #[cfg(debug_assertions)]
            println!(
                "{} ({}) {}",
                rock_i, rock_y, s[jet_i] as char
            );
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
            println!("{} ({}) v", rock_i, rock_y);
            last_y = rock_y;
            rock_y = rock_y.wrapping_sub(1);
        }
        rock_y = last_y;
        map.add_rock(rock, rock_y);
        #[cfg(debug_assertions)]
        println!(
            "{} ({}) {}",
            rock_i, rock_y, map.highest_rock
        );
        #[cfg(debug_assertions)]
        {
            let highest_rock = map.highest_rock + 1;
            for y in (highest_rock.saturating_sub(16)..=highest_rock).rev() {
                println!("|{}| {}", format!("{:07b}", map.get_row(y)).replace("0", " "), y + 1);
            }
        }
    }
    print!("{} ", map.highest_rock);
}
