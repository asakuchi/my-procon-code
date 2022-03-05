use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [(usize,usize,usize); n],
    }

    let total: Vec<_> = p.iter().map(|x| x.0 + x.1 + x.2).collect();

    let mut sorted = total.clone();
    sorted.sort_by_key(|&x| std::cmp::Reverse(x));

    for i in 0..n {
        if total[i] + 300 >= sorted[k - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
