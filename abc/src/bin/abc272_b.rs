use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: [[Usize1]; m],
    }

    for i in 0..n {
        for j in i + 1..n {
            let mut ok = false;

            for kai in 0..m {
                let mut one = false;
                let mut two = false;

                for &man in k[kai].iter() {
                    if man == i {
                        one = true;
                    }

                    if man == j {
                        two = true;
                    }
                }

                if one && two {
                    ok = true
                }
            }

            if !ok {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
