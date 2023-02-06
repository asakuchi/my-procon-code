use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [usize; n],
    }

    s.sort();

    let mut sum = 0;

    for &x in &s {
        sum += x;
    }

    if sum % 10 != 0 {
        println!("{}", sum);
        return;
    }

    for i in 0..n {
        if s[i] % 10 != 0 {
            println!("{}", sum - s[i]);
            return;
        }
    }

    println!("0");
}
