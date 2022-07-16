use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [Usize1; k],
    }

    let &max = a.iter().max().unwrap();

    let mut set = std::collections::HashSet::new();

    for i in 0..n {
        if a[i] == max {
            set.insert(i);
        }
    }

    for food in &b {
        if set.contains(food) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
