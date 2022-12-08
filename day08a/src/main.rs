use std::collections::HashSet;

pub fn main() {
    let b = include_bytes!("../input.txt");
    let l = b.iter().take_while(|x| **x != b'\n').count();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    let mut row_max = 0;
    let mut col_max: Vec<u8> = vec![0; l];
    b.iter()
        .take((l + 1) * (l + 1) - 1)
        .enumerate()
        .for_each(|(i, c)| {
            let (x, y) = (i % (l + 1), i / (l + 1));
            // println!("{} {} {}", x, y, c);
            match x {
                0 => row_max = 0,
                _ if x >= l => return,
                _ => {}
            }
            if *c > row_max {
                row_max = *c;
                visible.insert((x, y));
            }
            if *c > col_max[x] {
                col_max[x] = *c;
                visible.insert((x, y));
            }
        });
    col_max.iter_mut().for_each(|x| *x = 0);
    b.iter()
        .take((l + 1) * (l + 1) - 1)
        .rev()
        .enumerate()
        .for_each(|(i, c)| {
            let (x, y) = (i % (l + 1), i / (l + 1));
            // println!("{} {} {}", x, y, c);
            match x {
                0 => row_max = 0,
                _ if x >= l => return,
                _ => {}
            }
            if x >= l {
                return;
            }
            if *c > row_max {
                row_max = *c;
                visible.insert((l - x - 1, l - y - 1));
            }
            if *c > col_max[x] {
                col_max[x] = *c;
                visible.insert((l - x - 1, l - y - 1));
            }
        });

    #[cfg(debug_assertions)]
    {
        println!("{:?}", visible);
        let mut matrix: Vec<Vec<u8>> = vec![vec![0; l]; l];
        visible.iter().for_each(|(x, y)| matrix[*y][*x] = 1);
        matrix.iter().for_each(|l| println!("{:?}", l));
    };
    println!("{}", visible.len());
}
