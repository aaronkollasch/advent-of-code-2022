use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
struct DistMap {
    distances: Vec<i32>,
    w: usize,
    h: usize,
}

impl DistMap {
    pub fn get_val(&self, pos: Pos) -> i32 {
        self.distances[(pos.y as usize) * self.w + (pos.x as usize)]
    }

    pub fn set_val(&mut self, pos: Pos, val: i32) {
        self.distances[(pos.y as usize) * self.w + (pos.x as usize)] = val;
    }

    pub fn get_unset_neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut output = Vec::with_capacity(4);
        output.extend(
            [
                // (-1, -1),
                (-1, 0),
                // (-1, 1),
                (0, -1),
                (0, 1),
                // (1, -1),
                (1, 0),
                // (1, 1),
            ]
            .iter()
            .filter_map(|&p| {
                let new_pos = Pos {
                    x: pos.x + p.0,
                    y: pos.y + p.1,
                };
                if new_pos.x >= 0
                    && new_pos.x < self.w as i32
                    && new_pos.y >= 0
                    && new_pos.y < self.h as i32
                    && self.get_val(new_pos) == -1
                {
                    Some(new_pos)
                } else {
                    None
                }
            }),
        );
        output
    }
}

fn set_height(heights: &mut Vec<u8>, w: usize, pos: Pos, val: u8) {
    heights[(pos.y as usize) * w + (pos.x as usize)] = val;
}

fn get_height(heights: &Vec<u8>, w: usize, pos: Pos) -> u8 {
    heights[(pos.y as usize) * w + (pos.x as usize)]
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let w = s.iter().position(|&b| b == b'\n').unwrap();
    let h = s.split(|&b| b == b'\n').filter(|l| l.len() > 0).count();
    let mut heights = Vec::from_iter(s.iter().filter_map(|b| if *b >= b'A' {Some(*b)} else {None}));
    println!("w: {} h: {} l: {}", w, h, heights.len());
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    let mut distances: DistMap = DistMap {
        distances: vec![-1; w * h],
        w: w,
        h: h,
    };
    for (i, &b) in s.iter().enumerate() {
        match b {
            b'S' => {
                start.x = (i % (w + 1)) as i32;
                start.y = (i / (w + 1)) as i32;
            }
            b'E' => {
                end.x = (i % (w + 1)) as i32;
                end.y = (i / (w + 1)) as i32;
            }
            _ => {}
        }
    }
    println!("start: {} {}", start.x, start.y);
    println!("end: {} {}", end.x, end.y);
    set_height(&mut heights, w, start, b'a');
    set_height(&mut heights, w, end, b'z');
    println!("{} {}", get_height(&heights, w, start), get_height(&heights, w, end));

    let mut next_positions: HashSet<Pos> = HashSet::with_capacity(32);
    next_positions.insert(start);
    let mut distance = 0;

    while next_positions.len() > 0 {
        println!("distance {}", distance);
        println!("{:?}", next_positions);
        for p in next_positions.iter() {
            distances.set_val(*p, distance);
        }
        if next_positions.contains(&end) {
            println!("end");
            break;
        }
        let new_positions: HashSet<Pos> = HashSet::from_iter(
            next_positions.iter().map(|p| {
                println!("neighbors of {} {}: {:?}", p.x, p.y, distances.get_unset_neighbors(*p));
                println!("height of {} {}: {}", p.x, p.y, get_height(&heights, w, *p));
                distances.get_unset_neighbors(*p).into_iter().filter(|p2| {
                    println!("height of {} {}: {}", p2.x, p2.y, get_height(&heights, w, *p2));
                    get_height(&heights, w, *p2) as i8 - get_height(&heights, w, *p) as i8 <= 1
                })
            }).flatten());
        next_positions = new_positions;
        distance += 1;
    }
    print!("{} ", distances.get_val(start));
    print!("{} ", distances.get_val(end));
}
