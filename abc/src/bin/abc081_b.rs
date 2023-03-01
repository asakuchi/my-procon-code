use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut result = 0;

    loop {
        let mut ok = true;

        for &x in &a {
            if x % 2 != 0 {
                ok = false;
                break;
            }
        }

        if !ok {
            break;
        }

        result += 1;

        for i in 0..n {
            a[i] /= 2;
        }
    }

    println!("{}", result);
}
