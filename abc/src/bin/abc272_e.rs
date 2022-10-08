use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    }

    let mut num_sets = vec![HashSet::new(); m + 1];

    'i_loop: for i in 0..n {
        let mut start = 1;

        if a[i] < 0 {
            start = (a[i] / (i as isize + 1)).abs() as usize;
        }

        for stage in start..=m {
            let value = a[i] + (i as isize + 1) * (stage as isize);

            if value > n as isize {
                continue 'i_loop;
            }

            num_sets[stage].insert(value);
        }
    }

    'stage_loop: for stage in 1..=m {
        for i in 0..=n {
            if !num_sets[stage].contains(&(i as isize)) {
                println!("{}", i);
                continue 'stage_loop;
            }
        }
    }
}
