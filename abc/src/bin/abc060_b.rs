use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    for i in 1..1_000_000 {
        if (a * i) % b == c {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
