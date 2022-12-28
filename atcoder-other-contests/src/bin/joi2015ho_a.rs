use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; m],
        a_b_c: [(usize, usize, usize); n - 1],
    }

    let mut imos = vec![0isize; n];

    for i in 1..m {
        let bigger = p[i].max(p[i - 1]);
        let smaller = p[i].min(p[i - 1]);

        imos[smaller] += 1;
        imos[bigger] -= 1;
    }

    // println!("{:?}", imos);

    for i in 0..n {
        if i > 0 {
            imos[i] += imos[i - 1];
        }
    }

    // println!("{:?}", imos);

    let mut use_ic = vec![false; n - 1];

    for i in 0..n - 1 {
        let paper = a_b_c[i].0 * imos[i] as usize;
        let ic = a_b_c[i].1 * imos[i] as usize + a_b_c[i].2;

        if paper > ic {
            use_ic[i] = true;
        }
    }

    // println!("{:?}", use_ic);

    let mut cost = vec![0; n + 1];

    for i in 0..n - 1 {
        cost[i + 1] = cost[i] + if use_ic[i] { a_b_c[i].1 } else { a_b_c[i].0 };
    }

    // println!("{:?}", cost);

    let mut prev = p[0];

    let mut result = 0;

    // ICカード代
    for i in 0..n - 1 {
        if use_ic[i] {
            result += a_b_c[i].2;
        }
    }

    // 運賃
    for i in 1..m {
        let current = p[i];

        let bigger = prev.max(current);
        let smaller = prev.min(current);

        result += cost[bigger] - cost[smaller];

        prev = current;
    }

    println!("{}", result);
}
