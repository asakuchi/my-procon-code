use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        mut a: [usize; n],
    }

    a.sort_by_key(|price| std::cmp::Reverse(price % x));

    let mut a_discounted = Vec::with_capacity(n);

    for &price in a.iter() {
        if k > 0 && price > x {
            let count = k.min(price / x);
            k -= count;

            a_discounted.push(price - count * x);
        } else {
            a_discounted.push(price);
        }
    }

    let mut result = 0;

    for &price in a_discounted.iter() {
        if k > 0 {
            k -= 1;
        } else {
            result += price;
        }
    }

    println!("{}", result);
}
