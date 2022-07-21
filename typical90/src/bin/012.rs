use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut matrix = vec![vec![0; w + 1]; h + 1];

    let mut uf = UnionFind::new(h * w + w + 100);

    for _ in 0..q {
        input! {
            qi: usize,
        }

        if qi == 1 {
            input! {
                ri: usize,
                ci: usize,
            }

            matrix[ri][ci] = 1;

            for (diff_x, diff_y) in vec![(1isize, 0isize), (-1, 0), (0, 1), (0, -1)] {
                let (x, y) = (
                    (ri as isize + diff_x) as usize,
                    (ci as isize + diff_y) as usize,
                );

                if 1 <= x && x <= h && 1 <= y && y <= w && matrix[x][y] == 1 {
                    uf.union(ri * w + ci, x * w + y);
                }
            }
        } else {
            input! {
                rai: usize,
                cai: usize,
                rbi: usize,
                cbi: usize,
            }

            if matrix[rai][cai] == 1
                && matrix[rbi][cbi] == 1
                && uf.equiv(rai * w + cai, rbi * w + cbi)
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
