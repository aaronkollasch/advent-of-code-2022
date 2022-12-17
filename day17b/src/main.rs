#[cfg(debug_assertions)]
use kdam::{tqdm, BarExt};

// rocks:
// ####

// .#.
// ###
// .#.

// ..#
// ..#
// ###

// #
// #
// #
// #

// ##
// ##
//
const AIR: u8 = b' ';
const ROCK: u8 = b'#';

const ROCK_1: [[u8; 4]; 4] = [
    [ROCK, ROCK, ROCK, ROCK],
    [AIR, AIR, AIR, AIR],
    [AIR, AIR, AIR, AIR],
    [AIR, AIR, AIR, AIR],
];
const ROCK_2: [[u8; 4]; 4] = [
    [AIR, ROCK, AIR, AIR],
    [ROCK, ROCK, ROCK, AIR],
    [AIR, ROCK, AIR, AIR],
    [AIR, AIR, AIR, AIR],
];
const ROCK_3: [[u8; 4]; 4] = [
    [ROCK, ROCK, ROCK, AIR],
    [AIR, AIR, ROCK, AIR],
    [AIR, AIR, ROCK, AIR],
    [AIR, AIR, AIR, AIR],
];
const ROCK_4: [[u8; 4]; 4] = [
    [ROCK, AIR, AIR, AIR],
    [ROCK, AIR, AIR, AIR],
    [ROCK, AIR, AIR, AIR],
    [ROCK, AIR, AIR, AIR],
];
const ROCK_5: [[u8; 4]; 4] = [
    [ROCK, ROCK, AIR, AIR],
    [ROCK, ROCK, AIR, AIR],
    [AIR, AIR, AIR, AIR],
    [AIR, AIR, AIR, AIR],
];
const ROCKS: [[[u8; 4]; 4]; 5] = [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5];

// const MAP_HEIGHT: usize = 2022 * 4;
const MAP_HEIGHT: usize = 512;
const MAP_WIDTH: usize = 7;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Map {
    contents: [[u8; MAP_WIDTH]; MAP_HEIGHT],
    highest_rock: usize,
    map_height: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            contents: [[AIR; MAP_WIDTH]; MAP_HEIGHT],
            highest_rock: 0,
            map_height: 20,
        }
    }

    #[inline]
    pub fn get_contents(&self, pos: Pos) -> u8 {
        self.contents[pos.y % MAP_HEIGHT][pos.x]
    }

    #[inline]
    pub fn set_contents(&mut self, pos: Pos, val: u8) {
        self.contents[pos.y % MAP_HEIGHT][pos.x] = val;
    }

    #[inline]
    pub fn check_bounds(&self, pos: Pos) -> bool {
        pos.x < MAP_WIDTH && pos.y < self.map_height
    }

    pub fn collides_with(&self, rock: Rock) -> bool {
        let pos = rock.pos;
        let rock = ROCKS[rock.class];
        (0..4)
            .map(|x| (0..4).map(move |y| (x, y)))
            .flatten()
            .any(|(x, y)| {
                let map_pos: Pos = Pos {
                    x: pos.x.wrapping_add(x),
                    y: pos.y.wrapping_add(y),
                };
                if self.check_bounds(map_pos) {
                    rock[y][x] != AIR && self.get_contents(map_pos) != AIR
                } else {
                    rock[y][x] != AIR
                }
            })
    }

    pub fn add_rock(&mut self, rock: Rock) {
        let pos = rock.pos;
        let rock = ROCKS[rock.class];
        for x in 0..4 {
            for y in 0..4 {
                let map_pos: Pos = Pos {
                    x: pos.x.wrapping_add(x),
                    y: pos.y.wrapping_add(y),
                };
                if rock[y][x] != AIR {
                    self.set_contents(map_pos, rock[y][x]);
                }
            }
        }
        for y in self.highest_rock..self.highest_rock + 4 {
            if (0..MAP_WIDTH).any(|x| self.get_contents(Pos { x, y }) != AIR) {
                self.highest_rock = y + 1;
            }
        }
        for y in self.map_height..self.highest_rock + 20 {
            for x in 0..MAP_WIDTH {
                self.set_contents(Pos { x, y }, AIR);
            }
        }
        self.map_height = self.highest_rock + 20;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Rock {
    pub class: usize,
    pub pos: Pos,
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let jet_len = s.len() - 1;
    let mut jet_i = 0;
    let mut map = Map::new();
    #[cfg(debug_assertions)]
    println!("{}", jet_len);

    let n1 = 10000;
    #[cfg(debug_assertions)]
    let mut pb = tqdm!(total = n1);
    let mut heights = Vec::new();
    let mut last_highest = 0;
    let mut last_rock = 0;
    let mut first_rock_delta = 0;
    let mut first_height_delta = 0;
    let mut rock_delta = 0;
    let mut height_delta = 0;
    for i_rock in 0..n1 {
        let mut rock = Rock {
            class: i_rock % ROCKS.len(),
            pos: Pos {
                x: 2,
                y: map.highest_rock + 3,
            },
        };
        let mut last_pos = rock.pos;
        while !map.collides_with(rock) {
            last_pos = rock.pos;
            if s[jet_i] == b'<' {
                rock.pos.x = rock.pos.x.wrapping_sub(1);
            } else {
                rock.pos.x = rock.pos.x.wrapping_add(1);
            }
            jet_i = (jet_i + 1) % jet_len;
            if jet_i == 0 {
                if first_height_delta == 0 {
                    first_height_delta = map.highest_rock - last_highest;
                } else if height_delta != first_height_delta
                    && height_delta != map.highest_rock - last_highest
                {
                    panic!(
                        "mismatching height deltas {} {}",
                        height_delta,
                        map.highest_rock - last_highest
                    )
                }
                if first_rock_delta == 0 {
                    first_rock_delta = i_rock - last_rock;
                } else if rock_delta != first_rock_delta && rock_delta != i_rock - last_rock {
                    panic!(
                        "mismatching rock deltas {} {}",
                        rock_delta,
                        i_rock - last_rock
                    )
                }
                height_delta = map.highest_rock - last_highest;
                rock_delta = i_rock - last_rock;
                #[cfg(debug_assertions)]
                pb.write(format!(
                    "{} {} {}",
                    i_rock % ROCKS.len(),
                    rock_delta,
                    height_delta
                ));
                last_highest = map.highest_rock;
                last_rock = i_rock;
            }
            if map.collides_with(rock) {
                rock.pos = last_pos;
            }
            last_pos = rock.pos;
            rock.pos.y = rock.pos.y.wrapping_sub(1);
        }
        rock.pos = last_pos;
        map.add_rock(rock);
        heights.push(map.highest_rock);
        #[cfg(debug_assertions)]
        pb.update(1);
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
