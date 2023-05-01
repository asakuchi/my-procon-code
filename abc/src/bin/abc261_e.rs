use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        t_a: [(usize, usize); n],
    }

    let mut zero = 0;
    let mut one = !0;

    let mut x = c;

    for (t, a) in t_a {
        match t {
            1 => {
                zero &= a;
                one &= a;
            }
            2 => {
                zero |= a;
                one |= a;
            }
            _ => {
                zero ^= a;
                one ^= a;
            }
        }

        x = (x & one) | (!x & zero);

        println!("{}", x);
    }
}
