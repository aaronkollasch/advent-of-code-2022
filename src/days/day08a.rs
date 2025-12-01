use rustc_hash::FxHashSet;

const MAX_L: usize = 99;

pub fn get_result() -> usize {
    let b = include_bytes!("../../inputs/day08.txt");
    let l = b.iter().position(|x| *x == b'\n').unwrap();
    let mut visible: FxHashSet<usize> = Default::default();
    visible.reserve(l * l / 2);
    let mut row_max = 0;
    let mut col_max: [u8; MAX_L] = [0; MAX_L];
    b.iter()
        .take((l + 1) * l - 1)
        .enumerate()
        .for_each(|(i, c)| {
            let x = i % (l + 1);
            match x {
                0 => row_max = 0,
                _ if x >= l => return,
                _ => {}
            }
            if *c > row_max {
                row_max = *c;
                visible.insert(i);
            }
            if *c > col_max[x] {
                col_max[x] = *c;
                visible.insert(i);
            }
        });
    col_max.iter_mut().for_each(|x| *x = 0);
    b.iter()
        .take((l + 1) * l - 1)
        .enumerate()
        .rev()
        .for_each(|(i, c)| {
            let x = i % (l + 1);
            match x {
                _ if x == l - 1 => row_max = 0,
                _ if x >= l => return,
                _ => {}
            }
            if x >= l {
                return;
            }
            if *c > row_max {
                row_max = *c;
                visible.insert(i);
            }
            if *c > col_max[x] {
                col_max[x] = *c;
                visible.insert(i);
            }
        });

    #[cfg(debug_assertions)]
    {
        let mut matrix: Vec<Vec<u8>> = vec![vec![0; l]; l];
        visible.iter().for_each(|i| {
            let (x, y) = (*i % (l + 1), *i / (l + 1));
            matrix[y][x] = 1;
        });
        matrix.iter().for_each(|l| println!("{:?}", l));
    };
    return visible.len();
}

pub fn main() {
    print!("{} ", get_result());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        let result = get_result();
        assert_eq!(result, 1803);
    }
}
