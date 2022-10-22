use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut gen = vec![0; n * 2 + 2];

    for i in 0..n {
        let ameba = a[i];
        let ameba_gen = gen[ameba];

        gen[(i + 1) * 2] = ameba_gen + 1;
        gen[(i + 1) * 2 + 1] = ameba_gen + 1;
    }

    for i in 1..=2 * n + 1 {
        println!("{}", gen[i]);
    }
}
