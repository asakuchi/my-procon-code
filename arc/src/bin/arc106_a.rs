use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut max_a = 1;

    while 3usize.pow(max_a) < n {
        max_a += 1;
    }

    let mut max_b = 1;

    while 5usize.pow(max_b) < n {
        max_b += 1;
    }

    for a in 1..=max_a {
        for b in 1..=max_b {
            if 3usize.pow(a as u32) + 5usize.pow(b as u32) == n {
                println!("{} {}", a, b);
                return;
            }
        }
    }

    println!("-1");
}
