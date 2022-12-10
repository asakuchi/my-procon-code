use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: usize,
        a: [usize; n],
    }

    let total: usize = a.iter().sum();

    t %= total;

    for i in 0..n {
        let music = a[i];

        if t > music {
            t -= music;
        } else {
            println!("{} {}", i + 1, t);
            return;
        }
    }
}
