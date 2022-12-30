use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut org_a: [usize; n],
        mut k: [usize; q],
    }

    let mut a = vec![0];
    a.append(&mut org_a);

    a.push(2_000_000_000_000_000_001);

    let n = n + 2;

    let mut s = vec![0; n];

    for i in 1..n {
        s[i] = a[i] - i;
    }

    // println!("{:?}", s);

    let mut k: Vec<_> = k.iter().enumerate().collect();

    k.sort_by_key(|&(_i, value)| *value);

    let mut i = 1;

    let mut result = Vec::new();

    for (index, &k_i) in k {
        while k_i > s[i] {
            i += 1;
        }

        // println!("k_i {} a[i] {} a[i-1] {}", k_i, a[i - 1], a[i - 2]);

        result.push((index, a[i - 1] + k_i - s[i - 1]));
    }

    result.sort_by_key(|x| x.0);

    for (_index, value) in result {
        println!("{:?}", value);
    }
}
