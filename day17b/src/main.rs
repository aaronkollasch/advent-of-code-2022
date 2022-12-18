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

const MAP_HEIGHT: usize = 1024;
const MAP_CHUNKSIZE: usize = MAP_HEIGHT / 4;

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

#[inline]
#[no_mangle]
pub fn memzero(data: &mut [Row]) {
    for i in 0..data.len() {
        data[i] = 0;
    }
}

struct Map {
    contents: [Row; MAP_HEIGHT],
    highest_rock: usize,
    last_reset: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            contents: [0; MAP_HEIGHT],
            highest_rock: 0,
            last_reset: 0,
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

        if self.highest_rock - self.last_reset > MAP_CHUNKSIZE * 2 {
            let start_idx = self.last_reset % MAP_HEIGHT;
            memzero(&mut self.contents[start_idx..start_idx + MAP_CHUNKSIZE]);
            self.last_reset += MAP_CHUNKSIZE;
        }
    }
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let jet_len = s.len() - 1;
    #[cfg(debug_assertions)]
    println!("{}", jet_len);

    let mut map = Map::new();
    let mut jet_i = 0;
    let mut heights = Vec::with_capacity(16384);
    let mut last_highest = 0;
    let mut last_rock = 0;
    let mut first_rock_delta = 0;
    let mut first_height_delta = 0;
    let mut rock_delta = 0;
    let mut height_delta = 0;
    let mut i_rock = 0;
    let mut num_wraps = 0;
    loop {
        let mut rock = ROCKS[i_rock % ROCKS.len()];
        let mut last_rock_pos;
        let mut rock_y = map.highest_rock + 3;
        let mut last_y = rock_y;
        i_rock += 1;
        while rock_y != usize::MAX && !map.collides_with(rock, rock_y) {
            last_rock_pos = rock;
            if s[jet_i] == b'<' {
                shift_rock_left(&mut rock);
            } else {
                shift_rock_right(&mut rock);
            }
            jet_i = (jet_i + 1) % jet_len;
            if jet_i == 0 {
                num_wraps += 1;
                if num_wraps == 1 {
                    first_height_delta = map.highest_rock - last_highest;
                } else if num_wraps > 2 && height_delta != map.highest_rock - last_highest {
                    panic!(
                        "mismatching height deltas {} {}",
                        height_delta,
                        map.highest_rock - last_highest
                    )
                }
                if num_wraps == 1 {
                    first_rock_delta = i_rock - last_rock;
                } else if num_wraps > 2 && rock_delta != i_rock - last_rock {
                    panic!(
                        "mismatching rock deltas {} {}",
                        rock_delta,
                        i_rock - last_rock
                    )
                }
                height_delta = map.highest_rock - last_highest;
                rock_delta = i_rock - last_rock;
                last_highest = map.highest_rock;
                last_rock = i_rock;
            }
            if map.collides_with(rock, rock_y) {
                rock = last_rock_pos;
            }
            last_y = rock_y;
            rock_y = rock_y.wrapping_sub(1);
        }
        rock_y = last_y;
        map.add_rock(rock, rock_y);
        heights.push(map.highest_rock);
        if num_wraps >= 3 && i_rock >= last_rock + rock_delta - 1 {
            break;
        }
    }
    let n2 = 1000000000000;
    let last_rock_delta = (n2 - first_rock_delta - 1) % rock_delta;
    let n2_num_repeats = (n2 - first_rock_delta - last_rock_delta) / rock_delta;
    let n3 = last_rock + last_rock_delta;
    let last_height_delta = heights[n3] - last_highest;
    print!(
        "{} ",
        first_height_delta + n2_num_repeats * height_delta + last_height_delta
    );

    // run indefinitely for fun
    #[cfg(feature = "full_simulation")]
    {
        use kdam::{tqdm, BarExt};

        println!();
        let mut pb = tqdm!(total = n2);
        pb.set_description("simulating");
        let mut map = Map::new();
        let mut jet_i = 0;
        for i_rock in 0..n2 {
            let mut rock = ROCKS[i_rock % ROCKS.len()];
            let mut last_rock_pos;
            let mut rock_y = map.highest_rock + 3;
            let mut last_y = rock_y;
            while rock_y != usize::MAX && !map.collides_with(rock, rock_y) {
                last_rock_pos = rock;
                if s[jet_i] == b'<' {
                    shift_rock_left(&mut rock);
                } else {
                    shift_rock_right(&mut rock);
                }
                jet_i = (jet_i + 1) % jet_len;
                if map.collides_with(rock, rock_y) {
                    rock = last_rock_pos;
                }
                last_y = rock_y;
                rock_y = rock_y.wrapping_sub(1);
            }
            rock_y = last_y;
            map.add_rock(rock, rock_y);
            pb.update(1);
        }
        print!("{} ", map.highest_rock);
    }
}
