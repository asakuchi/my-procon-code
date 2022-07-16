use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    let mut list = std::collections::VecDeque::from(a);

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                list.swap(x, y);
            }
            2 => {
                input! {
                    _x: usize,
                    _y: usize,
                }
                let tmp = list.pop_back();
                if let Some(value) = tmp {
                    list.push_front(value);
                }
            }
            _ => {
                input! {
                    x: Usize1,
                    _y: usize,
                }
                println!("{}", list[x]);
            }
        }
    }
}
