use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }

    let mut log_k = 0;
    let mut x = 1;

    while x <= k {
        x *= 2;
        log_k += 1;
    }

    // doubling[k][i] : i番目から 2^k 進んだ町
    let mut doubling = vec![vec![0; n]; log_k + 1];

    for i in 0..n {
        doubling[0][i] = a[i];
    }

    for j in 0..log_k {
        for i in 0..n {
            doubling[j + 1][i] = doubling[j][doubling[j][i]];

            // println!(
            //     "i {i} j {j} 2^j {} doubling[j][i] {} doubling[j][doubling[j][i]] {} 2^j+1 {} doubling[j + 1][i] {}",
            //     2usize.pow(j as u32),
            //     doubling[j][i],
            //     doubling[j][doubling[j][i]],
            //     2usize.pow(j as u32 + 1),
            //     doubling[j + 1][i],
            // );
        }
    }

    // println!("{:?}", doubling);

    let mut current = 0;

    for i in (0..log_k).rev() {
        if k & (1 << i) > 0 {
            current = doubling[i][current];
        }
    }

    println!("{}", current + 1);
}
