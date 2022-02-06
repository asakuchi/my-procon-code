use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut p_index = -1;
    let mut q_index = -1;

    for (i, pair) in (1..=n).permutations(n).enumerate() {
        let mut p_same = true;
        let mut q_same = true;

        // println!("{:?} {:?} {:?} ", pair, p, q);

        for j in 0..n {
            if p_index == -1 {
                if pair[j] != p[j] {
                    p_same = false;
                }
            }
            if q_index == -1 {
                if pair[j] != q[j] {
                    q_same = false;
                }
            }
        }

        if p_index == -1 && p_same {
            p_index = i as isize;
        }
        if q_index == -1 && q_same {
            q_index = i as isize;
        }
    }

    // println!("{} {}", q_index, p_index);
    println!("{}", (q_index - p_index).abs());
}
