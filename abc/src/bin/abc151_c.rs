use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p_s: [(Usize1, String); m],
    }

    let mut ac_list = vec![false; n];
    let mut penalty_list = vec![0; n];

    let mut ac = 0;
    let mut penalty = 0;

    for (p, s) in p_s {
        if ac_list[p] {
            continue;
        }

        if s == "AC" {
            ac += 1;
            ac_list[p] = true;

            penalty += penalty_list[p];
        } else {
            penalty_list[p] += 1;
        }
    }

    println!("{} {}", ac, penalty);
}
