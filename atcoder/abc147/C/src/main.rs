use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut testimony = vec![Vec::new(); n];

    for i in 0..n {
        input! {
            a_count: usize,
            xy: [(Usize1, usize); a_count],
        }

        testimony[i] = xy;
    }

    let mut result = 0;

    for mask in 0..1 << n {
        let mut ok = true;

        let mut honest_count = 0;

        for i in 0..n {
            if mask & 1 << i > 0 {
                for &(x, y) in testimony[i].iter() {
                    if (y == 1 && mask & 1 << x > 0) || (y == 0 && mask & 1 << x == 0) {
                        // ok
                        // do nothing
                    } else {
                        ok = false;
                    }
                }

                honest_count += 1;
            }
        }

        if ok {
            result = result.max(honest_count);
        }
    }

    println!("{}", result);
}
