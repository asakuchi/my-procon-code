use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(isize, isize); n],
    }

    xy.sort();

    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (xy[i].0 - xy[k].0) * (xy[j].1 - xy[k].1)
                    - (xy[j].0 - xy[k].0) * (xy[i].1 - xy[k].1)
                    != 0
                {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
