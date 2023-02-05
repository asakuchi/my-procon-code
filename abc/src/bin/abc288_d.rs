use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
        q: usize,
        l_r: [(Usize1, usize); q],
    }

    let mut accum = vec![vec![0; n + 1]; k];

    for i in 0..n {
        for j in 0..k {
            accum[j][i + 1] += accum[j][i];
        }

        accum[i % k][i + 1] += a[i];
    }

    // println!("{:?}", accum);

    'q_loop: for (l, r) in l_r {
        // println!("{:?}", &a[l..r]);
        let s_0 = accum[0][r] - accum[0][l];

        for i in 1..k {
            if accum[i][r] - accum[i][l] != s_0 {
                println!("No");
                continue 'q_loop;
            }
        }

        println!("Yes");
    }
}
