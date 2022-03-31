use proconio::input;
use std::collections::VecDeque;

// #[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
    }

    let mut a = VecDeque::from(a);
    let mut b = VecDeque::from(b);

    'customer_loop: while let Some(customer) = b.pop_front() {
        while let Some(tako) = a.pop_front() {
            if tako <= customer && customer <= tako + t {
                continue 'customer_loop;
            } else {
                continue;
            }
        }

        println!("no");
        return;
    }

    println!("yes");
}
