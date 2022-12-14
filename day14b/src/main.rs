use itertools::Itertools;
use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

const MAP_AIR: u8 = b' ';
const MAP_ROCK: u8 = b'#';
const MAP_SAND: u8 = b'o';
const DROP_POS: Pos = Pos { x: 500, y: 0 };
const Y_MAX: usize = 157 + 2;
#[cfg(debug_assertions)]
const X_MIN_TRUE: usize = 491 - Y_MAX - 1;
const X_MIN: usize = 0;
const X_MAX: usize = 561 + Y_MAX + 1;
const MAP_WIDTH: usize = X_MAX - X_MIN + 1;
const MAP_HEIGHT: usize = Y_MAX + 1;
const MAP_SIZE: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(Debug, Clone)]
struct Map {
    contents: [u8; MAP_SIZE],
}

impl Map {
    pub fn new() -> Self {
        Self { contents: [MAP_AIR; MAP_SIZE] }
    }

    #[inline]
    pub fn get_val(&self, pos: Pos) -> u8 {
        self.contents[pos.y * MAP_WIDTH + pos.x]
    }

    #[inline]
    pub fn set_val(&mut self, pos: Pos, val: u8) {
        self.contents[pos.y * MAP_WIDTH + pos.x] = val;
    }

    #[inline]
    pub fn fill_line(&mut self, pos1: Pos, pos2: Pos, val: u8) {
        if pos1.x == pos2.x {
            for y in min(pos1.y, pos2.y)..=max(pos1.y, pos2.y) {
                self.set_val(Pos { x: pos1.x, y }, val);
            }
        } else if pos1.y == pos2.y {
            for x in min(pos1.x, pos2.x)..=max(pos1.x, pos2.x) {
                self.set_val(Pos { x, y: pos1.y }, val);
            }
        }
    }

    #[inline]
    pub fn drop_sand(&mut self, drop_path: &mut Vec<Pos>) -> bool {
        if drop_path.len() == 0 { return false; }
        loop {
            let pos = *drop_path.last().unwrap();
            if pos.y >= MAP_HEIGHT - 1 {
                return false;
            }
            let new_pos = Pos { x: pos.x, y: pos.y + 1};
            if self.get_val(new_pos) == MAP_AIR {
                drop_path.push(new_pos);
                continue;
            }
            let new_pos = Pos { x: pos.x - 1, y: pos.y + 1};
            if self.get_val(new_pos) == MAP_AIR {
                drop_path.push(new_pos);
                continue;
            }
            let new_pos = Pos { x: pos.x + 1, y: pos.y + 1};
            if self.get_val(new_pos) == MAP_AIR {
                drop_path.push(new_pos);
                continue;
            }
            self.set_val(pos, MAP_SAND);
            drop_path.pop();
            return true;
        }

    }
}

#[inline]
fn parse_point(p: &str) -> Pos {
    let (x, y) = p.split_once(',').unwrap();
    Pos {
        x: x.parse().unwrap_or_else(|_| panic!("parse failed x: {}", x)),
        y: y.parse().unwrap_or_else(|_| panic!("parse failed y: {}", y)),
    }
}

pub fn main() {
    let mut map = Map::new();
    let s = include_str!("../input.txt");
    let mut y_max = 0;
    #[cfg(debug_assertions)]
    let (mut x_min, mut x_max) = (usize::MAX, 0);
    s.lines().for_each(|l| {
        for (prev, next) in l.split(" -> ").map(parse_point).tuple_windows() {
            map.fill_line(prev, next, MAP_ROCK);
            y_max = max(y_max, max(prev.y, next.y));
            #[cfg(debug_assertions)]
            {
                x_min = min(x_min, min(prev.x, next.x));
                x_max = max(x_max, max(prev.x, next.x));
            }
        }
    });
    #[cfg(debug_assertions)]
    println!("y_max: {}, x_min: {}, x_max: {}", y_max, x_min, x_max);
    map.fill_line(Pos { x: X_MIN, y: y_max + 2 }, Pos { x: X_MAX, y: y_max + 2 }, MAP_ROCK);
    let mut i = 0;
    let mut drop_path: Vec<Pos> = Vec::with_capacity(MAP_HEIGHT);
    drop_path.push(DROP_POS);
    while map.drop_sand(&mut drop_path) { i += 1; }
    #[cfg(debug_assertions)]
    for y in 0..Y_MAX {
        for x in X_MIN_TRUE..X_MAX {
            print!("{}", char::from_u32(map.get_val(Pos { x, y: y }) as u32).unwrap());
        }
        println!();
    }
    print!("{} ", i);
}
