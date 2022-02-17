use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
        mut tx: [(usize, usize); q],
    }

    let n = 1048576;
    let mut a: Vec<isize> = vec![-1; n];

    let mut set = std::collections::BTreeSet::new();

    for i in 0..n {
        set.insert(i);
    }

    for &(t, x) in tx.iter() {
        if t == 1 {
            let i = x % n;
            let next = set.range(i..).next();

            match next {
                Some(&value) => {
                    a[value] = x as isize;
                    set.remove(&value);
                }
                None => {
                    let next = set.range(0..).next();
                    let next = *next.unwrap();
                    a[next] = x as isize;
                    set.remove(&next);
                }
            }
        } else {
            println!("{}", a[x % n]);
        }
    }
}
