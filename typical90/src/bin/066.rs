use proconio::input;

fn main() {
    input! {
        n: usize,
        l_r: [(usize, usize); n],
    }

    let mut result = 0.;

    for i in 0..n {
        let (l_i, r_i) = l_r[i];
        for j in i + 1..n {
            let (l_j, r_j) = l_r[j];

            let mut all_pattern = 0;
            let mut tentou = 0;

            for i_value in l_i..=r_i {
                for j_value in l_j..=r_j {
                    all_pattern += 1;

                    if i_value > j_value {
                        tentou += 1;
                    }
                }
            }

            result += tentou as f64 / all_pattern as f64;
        }
    }

    println!("{}", result);
}
