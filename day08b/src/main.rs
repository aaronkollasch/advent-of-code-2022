pub fn main() {
    let b = include_bytes!("../input.txt");
    let l = b.iter().take_while(|x| **x != b'\n').count();
    let mut scores: Vec<Vec<usize>> = vec![vec![1; l]; l];

    let mut row_idx: Vec<usize> = vec![0; 10];
    let mut col_idx: Vec<Vec<usize>> = vec![vec![0; 10]; l];
    b.iter()
        .take((l + 1) * (l + 1) - 1)
        .enumerate()
        .for_each(|(i, c)| {
            let (x, y) = (i % (l + 1), i / (l + 1));
            // println!("{} {} {}", x, y, c);
            match x {
                0 => {
                    row_idx.iter_mut().for_each(|x| *x = 0);
                }
                _ if x >= l => {
                    return;
                }
                _ => {}
            }
            let (view_x, view_y) = (
                x - row_idx[(c - b'0') as usize],
                y - col_idx[x][(c - b'0') as usize],
            );
            scores[y][x] *= view_x * view_y;
            row_idx
                .iter_mut()
                .take((c - b'0' + 1).into())
                .for_each(|idx| *idx = x);
            col_idx[x]
                .iter_mut()
                .take((c - b'0' + 1).into())
                .for_each(|idx| *idx = y);
        });
    col_idx
        .iter_mut()
        .for_each(|col| col.iter_mut().for_each(|x| *x = 0));
    b.iter()
        .take((l + 1) * (l + 1) - 1)
        .rev()
        .enumerate()
        .for_each(|(i, c)| {
            let (x, y) = (i % (l + 1), i / (l + 1));
            // println!("{} {} {}", x, y, c);
            match x {
                0 => {
                    row_idx.iter_mut().for_each(|x| *x = 0);
                }
                _ if x >= l => {
                    return;
                }
                _ => {}
            }
            let (view_x, view_y) = (
                x - row_idx[(c - b'0') as usize],
                y - col_idx[x][(c - b'0') as usize],
            );
            scores[l - y - 1][l - x - 1] *= view_x * view_y;
            row_idx
                .iter_mut()
                .take((c - b'0' + 1).into())
                .for_each(|idx| *idx = x);
            col_idx[x]
                .iter_mut()
                .take((c - b'0' + 1).into())
                .for_each(|idx| *idx = y);
        });

    // scores.iter().for_each(|l| println!("{:?}", l));
    println!(
        "{}",
        scores
            .iter()
            .map(|l| l.iter().max().unwrap())
            .max()
            .unwrap()
    );
}
