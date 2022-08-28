use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w];h],
    }

    let mut min_value = a[0][0];

    for i in 0..h {
        for j in 0..w {
            min_value = min_value.min(a[i][j]);
        }
    }

    let mut result = 0;

    for i in 0..h {
        for j in 0..w {
            result += a[i][j] - min_value;
        }
    }

    println!("{}", result);
}
