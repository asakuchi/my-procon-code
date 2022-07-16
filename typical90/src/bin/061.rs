use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut list = std::collections::VecDeque::with_capacity(q);

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }

        match t {
            1 => {
                list.push_front(x);
            }
            2 => {
                list.push_back(x);
            }
            _ => {
                println!("{}", list[x - 1]);
            }
        }
    }
}
