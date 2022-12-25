use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        l_r: [(usize, usize); m],
        p_q: [(usize, usize); q],
    }

    // 区間 [L,R] の列車の台数
    let mut count = vec![vec![0; n + 1]; n + 1];

    for (l, r) in l_r {
        count[l][r] += 1;
    }

    for l in 1..n + 1 {
        for r in 2..n + 1 {
            count[l][r] += count[l][r - 1];
        }
    }

    for (p, q) in p_q {
        let mut result = 0;

        for l in p..q + 1 {
            result += count[l][q];
        }

        println!("{}", result);
    }
}
