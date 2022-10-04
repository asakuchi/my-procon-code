use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        mut k: isize,
    }

    let mut result = 0;

    if k > a {
        result += a;
        k -= a;

        if k > b {
            k -= b;

            if k > c {
                result -= c;
                k -= c;
            } else {
                result -= k;
                k = 0;
            }
        } else {
            k = 0;
        }
    } else {
        result += k;
        k = 0;
    }

    println!("{}", result);
}
