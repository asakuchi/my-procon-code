use proconio::input;

fn main() {
    input! {
        n: usize,
        x_y: [(isize, isize); n]
    }

    let mut result = 0;

    for i in 0..n {
        for j in i + 1..n {
            let (xi, yi) = x_y[i];
            let (xj, yj) = x_y[j];

            let diff_x = xi - xj;
            let diff_y = yi - yj;

            if diff_x.abs() >= diff_y.abs() {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
