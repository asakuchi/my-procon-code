use proconio::input;

fn main() {
    input! {
        _l: usize,
        n_1: usize,
        n_2: usize,
        mut v_l_1: [(usize, usize); n_1],
        mut v_l_2: [(usize, usize); n_2],
    }

    let mut result = 0_usize;

    let mut left_1 = 0;
    let mut right_1 = 0;
    let mut left_2 = 0;
    let mut right_2 = 0;

    let mut value_1 = 0;
    let mut value_2 = 0;

    if let Some((v_1, l_1)) = v_l_1.pop() {
        right_1 = l_1;
        value_1 = v_1;
    }

    if let Some((v_2, l_2)) = v_l_2.pop() {
        right_2 = l_2;
        value_2 = v_2;
    }

    if value_1 == value_2 {
        result += right_1.min(right_2) - left_1.max(left_2);
    }

    while v_l_1.len() > 0 || v_l_2.len() > 0 {
        // 先に終わる方を更新する
        if right_1 < right_2 {
            let (v, l) = v_l_1.pop().unwrap();

            value_1 = v;
            left_1 = right_1;
            right_1 = left_1 + l;
        } else {
            let (v, l) = v_l_2.pop().unwrap();

            value_2 = v;
            left_2 = right_2;
            right_2 = left_2 + l;
        }

        if value_1 == value_2 {
            result += right_1.min(right_2) - left_1.max(left_2);
        }
    }

    println!("{}", result);
}
