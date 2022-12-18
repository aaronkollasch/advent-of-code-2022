use rustc_hash::FxHashSet;

type CubePos = i8;
type Pos3d = (CubePos, CubePos, CubePos);

pub fn main() {
    let s = include_str!("../input.txt");
    let mut num_cubes = 0;
    let mut sides: FxHashSet<Pos3d> = Default::default();
    sides.reserve(13000);
    s.lines().for_each(|l| {
        num_cubes += 1;
        let mut i_num = 0;
        let mut acc = 0;
        let mut result = [0; 3];
        for b in l.as_bytes().iter() {
            match *b {
                b'0'..=b'9' => {
                    acc = acc * 10 + (b - b'0') as CubePos;
                }
                b',' => {
                    result[i_num] = acc * 2;
                    i_num += 1;
                    acc = 0;
                }
                _ => {}
            }
        }
        result[i_num] = acc * 2;
        let (a, b, c) = (result[0], result[1], result[2]);
        sides.extend([
            (a + 1, b, c),
            (a, b + 1, c),
            (a, b, c + 1),
            (a - 1, b, c),
            (a, b - 1, c),
            (a, b, c - 1),
        ]);
    });
    let num_neighbors = num_cubes * 6 - sides.len();
    let num_surface = num_cubes * 6 - num_neighbors * 2;

    #[cfg(debug_assertions)]
    {
        println!("num cubes: {}", num_cubes);
        println!("num neighbors: {}", num_neighbors);
        println!("num sides: {}", num_cubes * 6);
        println!("num contacting sides: {}", num_neighbors * 2);
        println!("num surface sides: {}", num_surface);
    }

    print!("{} ", num_surface);
}
