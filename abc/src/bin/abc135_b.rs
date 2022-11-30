use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let list: Vec<_> = (1..=n).collect();

    let mut diff = 0;

    for i in 0..n {
        if p[i] != list[i] {
            diff += 1;
        }
    }

    if diff == 0 || diff == 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
