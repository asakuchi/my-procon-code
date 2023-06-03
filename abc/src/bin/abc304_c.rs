use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: isize,
        x_y: [(isize, isize); n],
    }

    let mut has_virus = vec![false; n];
    has_virus[0] = true;

    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(current) = queue.pop_front() {
        let p_1 = x_y[current];

        for next in 0..n {
            if has_virus[next] {
                continue;
            }

            if current == next {
                continue;
            }

            let p_2 = x_y[next];

            if (p_1.0 - p_2.0).pow(2) + (p_1.1 - p_2.1).pow(2) <= d * d {
                queue.push_back(next);
                has_virus[next] = true;
            }
        }
    }

    for i in 0..n {
        if has_virus[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
