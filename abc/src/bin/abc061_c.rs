use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a_b: [(usize, usize); n],
    }

    a_b.sort_by_key(|&(a, _b)| a);

    let mut count = 0;

    for (a, b) in a_b {
        count += b;

        if count >= k {
            println!("{}", a);
            return;
        }
    }
}
