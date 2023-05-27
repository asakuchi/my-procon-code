use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1; n]; m],
    }

    let mut list = vec![vec![true; n]; n];

    for i in 0..m {
        for j in 1..n {
            let x = a[i][j];
            let y = a[i][j - 1];

            list[x][y] = false;
            list[y][x] = false;
        }
    }

    let mut result = 0;

    for i in 0..n {
        for j in i + 1..n {
            if list[i][j] {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
