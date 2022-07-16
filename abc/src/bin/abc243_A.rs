use proconio::input;

fn main() {
    input! {
        mut v: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    loop {
        if v < a {
            println!("F");
            return;
        }

        v -= a;

        if v < b {
            println!("M");
            return;
        }

        v -= b;

        if v < c {
            println!("T");
            return;
        }

        v -= c;
    }
}
