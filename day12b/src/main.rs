use rustc_hash::FxHashSet;

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
    #[inline]
    pub fn get_val(&self, pos: Pos) -> i32 {
        self.distances[(pos.y as usize) * self.w + (pos.x as usize)]
    }

    #[inline]
    pub fn set_val(&mut self, pos: Pos, val: i32) {
        self.distances[(pos.y as usize) * self.w + (pos.x as usize)] = val;
    }

    #[inline]
    pub fn iter_unset_neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .iter()
            .filter_map(move |&p| {
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
            })
    }
}

#[inline]
fn set_height(heights: &mut Vec<u8>, w: usize, pos: Pos, val: u8) {
    heights[(pos.y as usize) * w + (pos.x as usize)] = val;
}

#[inline]
fn get_height(heights: &Vec<u8>, w: usize, pos: Pos) -> u8 {
    heights[(pos.y as usize) * w + (pos.x as usize)]
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let w = s.iter().position(|&b| b == b'\n').unwrap();
    let h = s.split(|&b| b == b'\n').filter(|l| l.len() > 0).count();
    let mut heights = Vec::from_iter(
        s.iter()
            .filter_map(|b| if *b >= b'A' { Some(*b) } else { None }),
    );
    #[cfg(debug_assertions)]
    eprintln!("w: {} h: {} l: {}", w, h, heights.len());
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
    set_height(&mut heights, w, start, b'a');
    set_height(&mut heights, w, end, b'z');
    #[cfg(debug_assertions)]
    {
        eprintln!("start: {} {}", start.x, start.y);
        eprintln!("end: {} {}", end.x, end.y);
        eprintln!(
            "{} {}",
            get_height(&heights, w, start),
            get_height(&heights, w, end)
        );
    }

    let mut next_positions: FxHashSet<Pos> = Default::default();
    let mut new_positions: FxHashSet<Pos> = Default::default();
    next_positions.insert(end);
    let mut distance = 0;

    while next_positions.len() > 0 {
        #[cfg(debug_assertions)]
        eprintln!("distance {}", distance);
        for p in next_positions.iter() {
            distances.set_val(*p, distance);
        }
        match next_positions
            .iter()
            .filter(|p| get_height(&heights, w, **p) == b'a')
            .next()
        {
            Some(p) => {
                #[cfg(debug_assertions)]
                eprintln!("end {}: {} {}", distance, p.x, p.y);
                start = *p;
                break;
            }
            _ => {}
        }
        new_positions.extend(
            next_positions
                .drain()
                .map(|p| {
                    let heights_ref = &heights;
                    let p_height = get_height(heights_ref, w, p) as i8;
                    #[cfg(debug_assertions)]
                    println!(
                        "*   height of [{}, {}]: {}",
                        p.x,
                        p.y,
                        get_height(heights_ref, w, p)
                    );
                    distances.iter_unset_neighbors(p).filter(move |p2| {
                        #[cfg(debug_assertions)]
                        println!(
                            " -> height of [{}, {}]: {}",
                            p2.x,
                            p2.y,
                            get_height(heights_ref, w, *p2)
                        );
                        get_height(heights_ref, w, *p2) as i8 - p_height >= -1
                    })
                })
                .flatten(),
        );
        (next_positions, new_positions) = (new_positions, next_positions);
        distance += 1;
    }
    #[cfg(debug_assertions)]
    println!("capacities: {} {}", next_positions.capacity(), new_positions.capacity());
    print!("{} ", distances.get_val(start));
}
