use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: usize,
        a: [usize; n-1],
        mut x_y: [(Usize1, usize); m],
    }

    let mut bonux = vec![0; n];

    for &(x, y) in &x_y {
        bonux[x] = y;
    }

    for i in 0..n - 1 {
        t += bonux[i];

        if t <= a[i] {
            println!("No");
            return;
        }

        t -= a[i];
    }

    println!("Yes");
}
