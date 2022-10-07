use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    }

    let mut buka = vec![0; n];

    for person in a {
        buka[person] += 1;
    }

    for i in 0..n {
        println!("{}", buka[i]);
    }
}
