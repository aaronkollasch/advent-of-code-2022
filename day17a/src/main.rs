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
