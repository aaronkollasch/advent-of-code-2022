use itertools::Itertools;
use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
struct Map {
    contents: Vec<u8>,
    w: usize,
    h: usize,
}

impl Map {
    #[inline]
    pub fn get_val(&self, pos: Pos) -> u8 {
        self.contents[(pos.y as usize) * self.w + (pos.x as usize)]
    }

    #[inline]
    pub fn set_val(&mut self, pos: Pos, val: u8) {
        self.contents[(pos.y as usize) * self.w + (pos.x as usize)] = val;
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
    pub fn drop_sand(&mut self, drop_pos: Pos) -> Option<Pos> {
        let mut pos = drop_pos.clone();
        loop {
            if pos.y >= MAX_HEIGHT - 1 {
                return None;
            }
            let new_pos = Pos { x: pos.x, y: pos.y + 1};
            if self.get_val(new_pos) == MAP_AIR {
                pos = new_pos;
                continue;
            }
            let new_pos = Pos { x: pos.x - 1, y: pos.y + 1};
            if self.get_val(new_pos) == MAP_AIR {
                pos = new_pos;
                continue;
            }
            let new_pos = Pos { x: pos.x + 1, y: pos.y + 1};
            if self.get_val(new_pos) == MAP_AIR {
                pos = new_pos;
                continue;
            }
            self.set_val(pos, MAP_SAND);
            return Some(pos);
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

const MAP_AIR: u8 = b' ';
const MAP_ROCK: u8 = b'#';
const MAP_SAND: u8 = b'o';
const DROP_POS: Pos = Pos { x: 500, y: 0 };
const MAX_WIDTH: usize = 800;
const MAX_HEIGHT: usize = 256;

pub fn main() {
    let w = MAX_WIDTH;
    let h = MAX_HEIGHT;
    let mut map = Map {
        contents: vec![MAP_AIR; w * h],
        w,
        h,
    };
    let s = include_str!("../input.txt");
    s.lines().for_each(|l| {
        for (prev, next) in l.split(" -> ").map(parse_point).tuple_windows() {
            map.fill_line(prev, next, MAP_ROCK);
        }
    });
    let mut i = 0;
    while map.drop_sand(DROP_POS).is_some() { i += 1; }
    #[cfg(debug_assertions)]
    for y in 0..h {
        for x in 0..w {
            print!("{}", char::from_u32(map.get_val(Pos { x, y: y }) as u32).unwrap());
        }
        println!();
    }
    print!("{} ", i);
}
