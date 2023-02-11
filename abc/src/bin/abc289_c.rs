use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1]; m],
    }

    let mut result = 0;

    for mask in 0..1 << m {
        // println!("{:b}", mask);

        let mut has = vec![false; n];

        for i in 0..m {
            if mask & 1 << i == 0 {
                continue;
            }

            for &value in a[i].iter() {
                has[value] = true;
            }
        }

        let mut ok = true;

        for value in 0..n {
            if !has[value] {
                ok = false;
                break;
            }
        }

        if ok {
            result += 1;
        }
    }

    println!("{}", result);
}
