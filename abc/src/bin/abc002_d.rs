// use proconio::fastout;
use proconio::input;

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut xy_org: [(usize, usize); m],
    }

    let xy: Vec<_> = xy_org.iter().map(|(x, y)| (x - 1, y - 1)).collect();

    let mut matrix = vec![vec![false; n]; n];

    for &(x, y) in xy.iter() {
        matrix[x][y] = true;
        matrix[y][x] = true;
    }

    let mut result = 0;

    'bitloop: for bit in 0..(1 << n) {
        let mut count = 0;

        // 立ってるビットをカウント
        for i in 0..n {
            if bit & (1 << i) > 0 {
                count += 1;
            }
        }

        if count <= result {
            continue;
        }

        for i in 0..n {
            if bit & (1 << i) == 0 {
                continue;
            }
            for j in 0..n {
                if i == j {
                    continue;
                }
                if bit & (1 << j) == 0 {
                    continue;
                }

                if !matrix[i][j] {
                    continue 'bitloop;
                }
            }
        }

        result = count;
    }

    println!("{}", result);
}
