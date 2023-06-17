use proconio::input;

fn main() {
    input! {
        a: [usize; 64],
    }

    let mut result = 0_u128;

    let mut p = 1;

    for i in 0..a.len() {
        if a[i] == 1 {
            result += p;
        }

        p *= 2;
    }

    println!("{}", result);
}
