use proconio::fastout;
use proconio::input;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            command : usize,
        }

        match command {
            1 => {
                input! {
                    x : usize,
                    c : usize,
                }

                match queue.pop_back() {
                    Some((num, count)) => {
                        if num == x {
                            queue.push_back((x, c + count));
                        } else {
                            queue.push_back((num, count));
                            queue.push_back((x, c));
                        }
                    }
                    None => {
                        queue.push_back((x, c));
                    }
                }
            }
            _ => {
                input! {
                    mut c : usize,
                }

                let mut sum = 0;

                while let Some((num, count)) = queue.pop_front() {
                    if c >= count {
                        sum += num * count;
                        c -= count;
                    } else {
                        sum += num * c;

                        let diff = count - c;
                        queue.push_front((num, diff));

                        c = 0;
                    }

                    if c == 0 {
                        break;
                    }
                }

                println!("{}", sum);
            }
        }
    }
}
