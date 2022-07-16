use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: isize,
        a: isize,
        d: isize,
        n: isize,
    }

    // 末項
    let al = a + (n - 1) * d;

    if a <= al {
        if x <= a {
            println!("{}", a - x);
            return;
        } else if x >= al {
            println!("{}", x - al);
            return;
        }
    }

    if a >= al {
        if x <= al {
            println!("{}", al - x);
            return;
        } else if x >= a {
            println!("{}", x - a);
            return;
        }
    }

    if d == 0 {
        if a <= al {
            if x - a > al - x {
                println!("{}", al - x);
                return;
            } else {
                println!("{}", x - a);
                return;
            }
        } else {
            if x - al > a - x {
                println!("{}", a - x);
                return;
            } else {
                println!("{}", x - al);
                return;
            }
        }
    }

    let mut modu = (x - a) % d.abs();

    if modu < 0 {
        modu += d.abs();
    }

    let result = modu.min(d.abs() - modu);

    println!("{}", result);
}
