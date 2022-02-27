use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut x: usize = 0;

    let mut first_index = vec![-1; n];

    let mut loop_size = n;

    for index in 1..n + 1 {
        let x_mod_n = x % n;
        x += a[x_mod_n];
        k -= 1;

        if k == 0 {
            println!("{}", x);
            return;
        }

        if first_index[x_mod_n] != -1 {
            loop_size = index - first_index[x_mod_n] as usize;
        } else {
            first_index[x_mod_n] = index as isize;
        }
    }

    while k % loop_size != 0 {
        k -= 1;
        x += a[x % n];
    }

    if k == 0 {
        println!("{}", x);
        return;
    }

    let mut loop_count = 0;

    for _ in 0..loop_size {
        k -= 1;
        x += a[x % n];

        loop_count += a[x % n];
    }

    x += loop_count * (k / loop_size);

    println!("{}", x);
}
