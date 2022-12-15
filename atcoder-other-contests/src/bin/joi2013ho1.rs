use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }

    let mut c_deq = VecDeque::from(c);

    let mut list = Vec::new();

    while let Some(on_off) = c_deq.pop_front() {
        let mut prev = on_off;
        let mut size = 1;

        while let Some(next) = c_deq.pop_front() {
            if prev != next {
                size += 1;
                prev = next;
            } else {
                c_deq.push_front(next);
                break;
            }
        }

        list.push(size);
    }

    if list.len() <= 3 {
        let result: usize = list.iter().sum();
        println!("{}", result);
    } else {
        let mut result = 0;

        for i in 2..list.len() {
            result = result.max(list[i - 2..=i].iter().sum::<usize>())
        }

        println!("{}", result);
    }
}
