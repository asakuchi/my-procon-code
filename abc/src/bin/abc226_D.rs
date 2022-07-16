use num::integer::Integer;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(isize, isize); n],
    }

    let mut magic = std::collections::HashSet::new();

    for i in 0..n {
        let (p1_x, p1_y) = xy[i];

        for j in 0..n {
            if i == j {
                continue;
            }

            let (p2_x, p2_y) = xy[j];

            let x = p2_x - p1_x;
            let y = p2_y - p1_y;

            let gcd = x.gcd(&y);

            magic.insert((x / gcd, y / gcd));
        }
    }

    println!("{}", magic.len());
}
