use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
        k: usize,
        c_d: [(Usize1, Usize1); k],
    }

    let mut result = 0;

    for mask in 0..(1 << k) {
        let mut placed = vec![false; n];

        for i in 0..k {
            if mask & 1 << i > 0 {
                // C に置く
                let c = c_d[i].0;

                placed[c] = true;
            } else {
                // D に置く
                let d = c_d[i].1;

                placed[d] = true;
            }
        }

        let mut dishes = 0;

        for &(a, b) in &a_b {
            if placed[a] && placed[b] {
                dishes += 1;
            }
        }

        result = result.max(dishes);
    }

    println!("{}", result);
}
