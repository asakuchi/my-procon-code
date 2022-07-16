// use proconio::fastout;
use proconio::input;

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n],
    }

    let mut first = a[0];
    let mut second = b[0];

    for i in 1..n {
        // eprintln!("pre {} {} {} {} {}", i, first, second, a[i], b[i]);

        let next_first;
        let next_second;

        if (first != -1 && (first - a[i]).abs() <= k)
            || (second != -1 && (second - a[i]).abs() <= k)
        {
            next_first = a[i];
        } else {
            next_first = -1;
        }

        if (first != -1 && (first - b[i]).abs() <= k)
            || (second != -1 && (second - b[i]).abs() <= k)
        {
            next_second = b[i];
        } else {
            next_second = -1;
        }

        // eprintln!("post {} {} {} {} {}", i, first, second, a[i], b[i]);

        if next_first == -1 && next_second == -1 {
            println!("No");
            return;
        }

        first = next_first;
        second = next_second;
    }

    println!("Yes");
}
