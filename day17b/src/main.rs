type Row = u8;
type Rock = ([Row; 4], usize);

// rocks:
//
// ####
const ROCK_1: Rock = ([0b0011110, 0, 0, 0], 1);
// .#.
// ###
// .#.
const ROCK_2: Rock = ([0b0001000, 0b0011100, 0b0001000, 0], 3);
// ..#
// ..#
// ###
const ROCK_3: Rock = ([0b0011100, 0b0000100, 0b0000100, 0], 3);
// #
// #
// #
// #
const ROCK_4: Rock = ([0b0010000, 0b0010000, 0b0010000, 0b0010000], 4);
// ##
// ##
const ROCK_5: Rock = ([0b0011000, 0b0011000, 0, 0], 2);

const ROCKS: [Rock; 5] = [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5];

const MAP_HEIGHT: usize = 128;

fn shift_rock_left(rock: &mut Rock) {
    if (0..rock.1).map(|i_rock| {
        rock.0[i_rock] & 0b1000000
    }).fold(0, |acc, x| acc | x as Row) > 0 {
        return
    }
    for i_rock in 0..rock.1 {
        rock.0[i_rock] <<= 1;
    }
}

fn shift_rock_right(rock: &mut Rock) {
    if (0..rock.1).map(|i_rock| {
        rock.0[i_rock] & 0b0000001
    }).fold(0, |acc, x| acc | x as Row) > 0 {
        return
    }
    for i_rock in 0..rock.1 {
        rock.0[i_rock] >>= 1;
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
        (rock_y..rock_y + rock.1)
            .enumerate()
            .any(|(i_rock, y)| {
                self.get_row(y) & rock.0[i_rock] > 0
            })
    }

    pub fn add_rock(&mut self, rock: Rock, rock_y: usize) {
        (rock_y..rock_y + rock.1)
            .enumerate()
            .for_each(|(i_rock, y)| {
                let row = self.get_row_mut(y);
                *row |= rock.0[i_rock];
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
    #[cfg(debug_assertions)]
    println!("{}", jet_len);

    let mut heights = Vec::new();
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
        while !map.collides_with(rock, rock_y) {
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
                } else if num_wraps > 2
                    && height_delta != map.highest_rock - last_highest
                {
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
        if num_wraps >= 3 && i_rock >= last_rock + rock_delta - 1 { break; }
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
}
