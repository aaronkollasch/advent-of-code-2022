#[cfg(debug_assertions)]
use std::cmp::max;
use std::cmp::min;

type Pos = usize;

#[inline]
fn pos(x: usize, y: usize) -> Pos {
    y * MAP_WIDTH + x
}

#[inline]
fn x(p: Pos) -> usize {
    p % MAP_WIDTH
}

#[inline]
fn y(p: Pos) -> usize {
    p / MAP_WIDTH
}

const MAP_AIR: u8 = b' ';
const MAP_ROCK: u8 = b'#';
const MAP_SAND: u8 = b'o';
const Y_MAX: usize = 157 + 2;
const X_MIN: usize = 491 - 1;
const X_MAX: usize = 561 + 1;
const MAP_WIDTH: usize = X_MAX - X_MIN + 1;
const MAP_HEIGHT: usize = Y_MAX + 1;
const MAP_SIZE: usize = MAP_WIDTH * MAP_HEIGHT;
const DROP_POS: Pos = 500 - X_MIN;

#[derive(Debug, Clone)]
struct Map {
    contents: [u8; MAP_SIZE],
}

impl Map {
    pub fn new() -> Self {
        Self {
            contents: [MAP_AIR; MAP_SIZE],
        }
    }

    #[inline]
    pub fn get_val(&self, pos: Pos) -> u8 {
        self.contents[pos]
    }

    #[inline]
    pub fn set_val(&mut self, pos: Pos, val: u8) {
        self.contents[pos] = val;
    }

    #[inline]
    pub fn fill_line(&mut self, pos1: Pos, pos2: Pos, val: u8) {
        if x(pos1) == x(pos2) {
            let pos = min(pos1, pos2);
            for y in 0..=y(pos1.abs_diff(pos2)) {
                self.set_val(pos + y * MAP_WIDTH, val);
            }
        } else if y(pos1) == y(pos2) {
            let pos = min(pos1, pos2);
            for x in 0..=pos1.abs_diff(pos2) {
                self.set_val(pos + x, val);
            }
        }
    }

    #[inline]
    pub fn fill_sand(&mut self, drop_pos: Pos) -> u32 {
        let mut i = 0;
        let mut drop_path: Vec<Pos> = Vec::with_capacity(MAP_HEIGHT);
        drop_path.push(drop_pos);
        while drop_path.len() > 0 {
            let pos = *drop_path.last().unwrap();
            if y(pos) >= MAP_HEIGHT - 1 {
                break;
            }
            let mut new_pos = pos + MAP_WIDTH;
            if self.get_val(new_pos) == MAP_AIR {
                drop_path.push(new_pos);
                continue;
            }
            new_pos -= 1;
            if self.get_val(new_pos) == MAP_AIR {
                drop_path.push(new_pos);
                continue;
            }
            new_pos += 2;
            if self.get_val(new_pos) == MAP_AIR {
                drop_path.push(new_pos);
                continue;
            }
            self.set_val(pos, MAP_SAND);
            drop_path.pop();
            i += 1;
        }
        i
    }
}

pub fn main() {
    let mut map = Map::new();
    #[cfg(debug_assertions)]
    let (mut y_max, mut x_min, mut x_max) = (0, usize::MAX, 0);
    let mut prev: Option<Pos> = None;
    let (mut pair_0, mut acc) = (0, 0);
    let mut pair_idx: bool = false;
    let s = include_bytes!("../input.txt");
    s.into_iter().for_each(|b| match b {
        b' ' if pair_idx => {
            let next = pos(pair_0 - X_MIN, acc);
            if prev.is_some() {
                map.fill_line(prev.unwrap(), next, MAP_ROCK);
            }
            prev = Some(next);
            #[cfg(debug_assertions)]
            {
                y_max = max(y_max, y(next));
                x_min = min(x_min, x(next));
                x_max = max(x_max, x(next));
            }
            acc = 0;
            pair_idx = false;
        }
        b' ' => {}
        b',' => {
            pair_0 = acc;
            acc = 0;
            pair_idx = true;
        }
        b'\n' => {
            let next = pos(pair_0 - X_MIN, acc);
            if prev.is_some() {
                map.fill_line(prev.unwrap(), next, MAP_ROCK);
            }
            #[cfg(debug_assertions)]
            {
                y_max = max(y_max, y(next));
                x_min = min(x_min, x(next));
                x_max = max(x_max, x(next));
            }
            prev = None;
            acc = 0;
            pair_idx = false;
        }
        b'0'..=b'9' => {
            acc = acc * 10 + (b - b'0') as Pos;
        }
        _ => {}
    });
    #[cfg(debug_assertions)]
    println!("y_max: {}, x_min: {}, x_max: {}", y_max, x_min, x_max);
    let i = map.fill_sand(DROP_POS);
    #[cfg(debug_assertions)]
    {
        let (mut y_max, mut x_min, mut x_max) = (0, usize::MAX, 0);
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let v = map.get_val(pos(x, y));
                print!("{}", char::from_u32(v as u32).unwrap());
                if v != MAP_AIR {
                    y_max = max(y_max, y);
                    x_max = max(x_max, x);
                    x_min = min(x_min, x);
                }
            }
            println!();
        }
        println!("y_max: {}, x_min: {}, x_max: {}", y_max, x_min + X_MIN, x_max + X_MIN);
    }
    print!("{} ", i);
}
