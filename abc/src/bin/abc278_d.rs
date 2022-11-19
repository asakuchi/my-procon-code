use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut list = Vec::with_capacity(n);

    for i in 0..n {
        list.push((-1, a[i]));
    }

    let mut assign = (-1, 0);

    for current_time in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                x: usize,
            }

            assign = (current_time as isize, x);
        } else if query == 2 {
            input! {
                i: Usize1,
                x: usize,
            }

            let (time, value) = list[i];

            if time >= assign.0 {
                // 前回の代入より新しい
                list[i] = (current_time as isize, value + x);
            } else {
                // 古い
                list[i] = (current_time as isize, x);
            }
        } else {
            input! {
                i: Usize1,
            }

            let (time, value) = list[i];

            if time >= assign.0 {
                // 前回の代入より新しい
                println!("{}", value + assign.1);
            } else {
                // 古い
                println!("{}", assign.1);
            }
        }
    }
}
