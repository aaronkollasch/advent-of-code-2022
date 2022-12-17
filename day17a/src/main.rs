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
    [ ROCK, ROCK, ROCK, ROCK ],
    [ AIR, AIR, AIR, AIR ],
    [ AIR, AIR, AIR, AIR ],
    [ AIR, AIR, AIR, AIR ],
];
const ROCK_2: [[u8; 4]; 4] = [
    [ AIR, ROCK, AIR, AIR ],
    [ ROCK, ROCK, ROCK, AIR ],
    [ AIR, ROCK, AIR, AIR ],
    [ AIR, AIR, AIR, AIR ],
];
const ROCK_3: [[u8; 4]; 4] = [
    [ ROCK, ROCK, ROCK, AIR ],
    [ AIR, AIR, ROCK, AIR ],
    [ AIR, AIR, ROCK, AIR ],
    [ AIR, AIR, AIR, AIR ],
];
const ROCK_4: [[u8; 4]; 4] = [
    [ ROCK, AIR, AIR, AIR ],
    [ ROCK, AIR, AIR, AIR ],
    [ ROCK, AIR, AIR, AIR ],
    [ ROCK, AIR, AIR, AIR ],
];
const ROCK_5: [[u8; 4]; 4] = [
    [ ROCK, ROCK, AIR, AIR ],
    [ ROCK, ROCK, AIR, AIR ],
    [ AIR, AIR, AIR, AIR ],
    [ AIR, AIR, AIR, AIR ],
];
const ROCKS: [[[u8; 4]; 4]; 5] = [
    ROCK_1,
    ROCK_2,
    ROCK_3,
    ROCK_4,
    ROCK_5,
];

const MAP_HEIGHT: usize = 2022 * 4;
const MAP_WIDTH: usize = 7;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Map {
    contents: [[u8; MAP_WIDTH]; MAP_HEIGHT],
    highest_rock: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            contents: [[AIR; MAP_WIDTH]; MAP_HEIGHT],
            highest_rock: 0,
        }
    }

    #[inline]
    pub fn check_bounds(&self, pos: Pos) -> bool {
        pos.x < MAP_WIDTH && pos.y < MAP_HEIGHT
    }

    pub fn collides_with(&self, rock: Rock) -> bool {
        let pos = rock.pos;
        let rock = ROCKS[rock.class];
        (0..4)
            .map(|x| (0..4).map(move |y| (x, y)))
            .flatten()
            .any(|(x, y)| {
                let map_pos: Pos = Pos { x: pos.x.wrapping_add(x), y: pos.y.wrapping_add(y) };
                if self.check_bounds(map_pos) {
                    rock[y][x] != AIR && self.contents[map_pos.y][map_pos.x] != AIR
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
                let map_pos: Pos = Pos { x: pos.x.wrapping_add(x), y: pos.y.wrapping_add(y) };
                if rock[y][x] != AIR {
                    self.contents[map_pos.y][map_pos.x] = rock[y][x];
                }
            }
        }
        for y in self.highest_rock..self.highest_rock+4 {
            if (0..MAP_WIDTH).any(|x| self.contents[y][x] != AIR) {
                self.highest_rock = y + 1;
            }
        }
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

    for i_rock in 0..2022 {
        let mut rock = Rock { class: i_rock % ROCKS.len(), pos: Pos { x: 2, y: map.highest_rock + 3 } };
        let mut last_pos = rock.pos;
        println!("{} ({}, {}) {}", rock.class, rock.pos.x, rock.pos.y, map.highest_rock);
        while !map.collides_with(rock) {
            println!("{} ({}, {}) {}", rock.class, rock.pos.x, rock.pos.y, s[jet_i] as char);
            last_pos = rock.pos;
            if s[jet_i] == b'<' {
                rock.pos.x = rock.pos.x.wrapping_sub(1);
            } else {
                rock.pos.x = rock.pos.x.wrapping_add(1);
            }
            jet_i = (jet_i + 1) % jet_len;
            if map.collides_with(rock) {
                rock.pos = last_pos;
            }
            println!("{} ({}, {}) v", rock.class, rock.pos.x, rock.pos.y);
            last_pos = rock.pos;
            rock.pos.y = rock.pos.y.wrapping_sub(1);
        }
        rock.pos = last_pos;
        map.add_rock(rock);
        println!("{} ({}, {}) {}", rock.class, rock.pos.x, rock.pos.y, map.highest_rock);
        // {
        //     for y in 0..map.highest_rock {
        //         let y = map.highest_rock - y - 1;
        //         print!("|");
        //         for x in 0..MAP_WIDTH {
        //             print!("{}", map.contents[y][x] as char);
        //         }
        //         println!("|");
        //     }
        // }
    }
    print!("{} ", map.highest_rock);
}
