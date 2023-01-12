use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [Usize1; q],
    }

    let mut points = vec![0; n];

    for x in a {
        points[x] += 1;
    }

    // println!("{:?}", points);

    for point in points {
        // 失点
        if q - point >= k {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
