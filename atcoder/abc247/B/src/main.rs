// use proconio::fastout;
use proconio::input;

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut st: [(String, String); n],
    }

    for i in 0..n {
        let mut sei = true;
        let mut mei = true;

        for j in 0..n {
            if i == j {
                continue;
            }

            if st[i].0 == st[j].0 || st[i].0 == st[j].1 {
                mei = false;
            }

            if st[i].1 == st[j].0 || st[i].1 == st[j].1 {
                sei = false;
            }
        }

        if !mei && !sei {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
