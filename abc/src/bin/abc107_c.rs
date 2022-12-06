use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [isize; n],
    }

    let mut result;

    let mut left_is_positive = x[0] > 0;
    let mut right_is_positive = x[k - 1] > 0;

    if !left_is_positive && !right_is_positive {
        result = x[0].abs();
    } else if !left_is_positive && right_is_positive {
        // 左端を往復
        result = x[0].abs() * 2 + x[k - 1].abs();

        // 右端を往復
        result = result.min(x[0].abs() + x[k - 1].abs() * 2);
    } else {
        result = x[k - 1].abs();
    }

    // println!("{} {} {}", left_is_positive, right_is_positive, result);

    for i in k..n {
        let right = i;
        let left = i - k + 1;

        if x[left] > 0 {
            left_is_positive = true;
        }

        if x[right] > 0 {
            right_is_positive = true;
        }

        let score;

        if !left_is_positive && !right_is_positive {
            score = x[left].abs();
        } else if !left_is_positive && right_is_positive {
            // 左端を往復
            // 右端を往復
            score = (x[left].abs() * 2 + x[right].abs()).min(x[left].abs() + x[right].abs() * 2);
        } else {
            score = x[right].abs();
        }

        result = result.min(score);

        // println!("left {} right {}", left, right);
        // println!("{} {} {}", left_is_positive, right_is_positive, result);
    }

    println!("{}", result);
}
