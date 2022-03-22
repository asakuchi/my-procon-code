use permutohedron::LexicalPermutation as _;
use proconio::input;
use proconio::marker::Usize1;

const MAX_VALUE: usize = 1_000_000_000;

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        rumor: [(Usize1, Usize1); m],
    }

    let mut matrix = vec![vec![true; n]; n];

    for &(x, y) in rumor.iter() {
        matrix[x][y] = false;
        matrix[y][x] = false;
    }

    let mut result = MAX_VALUE;

    let mut values: Vec<_> = (0..n).collect();

    loop {
        let mut ok = true;

        for j in 1..n {
            if !matrix[values[j]][values[j - 1]] {
                ok = false;
            }
        }

        if ok {
            let mut time = 0;

            for j in 0..n {
                time += a[values[j]][j];
            }

            result = result.min(time);
        }

        if !values.next_permutation() {
            break;
        }
    }

    if result != MAX_VALUE {
        println!("{}", result);
    } else {
        println!("-1");
    }
}
