use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        t_a: [(usize, usize); n],
    }

    let mut all_on = (1 << 30) - 1;
    let mut all_off = 0;

    // println!(" all_on {:030b}", all_on);
    // println!("all_off {:030b}", all_off);

    let mut x = c;

    // println!("      c {:030b}", c);

    for i in 0..n {
        let (t, a) = t_a[i];

        let mut next_x = 0;

        if t == 1 {
            // println!("and");

            all_on &= a;
            all_off &= a;
        } else if t == 2 {
            // println!("or");

            all_on |= a;
            all_off |= a;
        } else {
            // println!("xor");

            all_on ^= a;
            all_off ^= a;
        }

        // println!("      a {:030b}", a);

        // println!(" all_on {:030b}", all_on);
        // println!("all_off {:030b}", all_off);

        for k in 0..30 {
            if x & 1 << k == 0 {
                next_x |= all_off & 1 << k;
            } else {
                next_x |= all_on & 1 << k;
            }
        }

        x = next_x;

        // println!("      x {:030b}", x);

        println!("{}", x);
    }
}
