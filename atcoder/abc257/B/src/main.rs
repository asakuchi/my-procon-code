use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q],
    }

    let mut list = vec![false; n];

    for i in 0..k {
        list[a[i] - 1] = true;
    }

    for i in 0..q {
        let order = l[i];

        if a[order - 1] == n {
            continue;
        }

        if list[a[order - 1]] {
            continue;
        }

        list[a[order - 1]] = true;
        list[a[order - 1] - 1] = false;

        a[order - 1] += 1;
    }

    let result = a
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
}
