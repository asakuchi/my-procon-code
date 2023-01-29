use proconio::input;

fn main() {
    input! {
        c: [[isize; 3]; 3],
    }

    for i in 0..3 {
        for j in i + 1..3 {
            let a_i_minus_a_j = c[j][0] - c[i][0];

            for k in 0..3 {
                if c[j][k] - c[i][k] != a_i_minus_a_j {
                    println!("No");
                    return;
                }
            }
        }
    }

    for i in 0..3 {
        for j in i + 1..3 {
            let b_i_minus_b_j = c[0][j] - c[0][i];

            for k in 0..3 {
                if c[k][j] - c[k][i] != b_i_minus_b_j {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
