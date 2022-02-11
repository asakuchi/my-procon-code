// use proconio::fastout;
use proconio::input;

// #[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by_key(|x| x.1);

    // println!("{:?}", lr);

    // とりあえず1回パンチ
    let mut result = 1;

    let mut s = lr[0].1;
    let mut t = lr[0].1 + d - 1;
    // println!("{} {} に1stパンチ", s, t);

    'outer: for (l, r) in lr {
        while s <= r && t >= l {
            // すでに放ったパンチで壊せている
            // println!("{} {} は壊せている", l, r);
            continue 'outer;
        }

        s = r;
        t = r + d - 1;
        result += 1;
        // println!("{} {} にパンチ", s, t);
    }

    println!("{}", result);
}
