// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 001 - Yokan Party（★4）
///
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }

    let cutting_point = [vec![0], a.clone(), vec![l]].concat();

    let mut p = vec![0; n + 1];

    for i in 0..n + 1 {
        p[i] = cutting_point[i + 1] - cutting_point[i];
    }

    // println!("cutting_point:{:?}", cutting_point);
    // println!("p:{:?}", p);

    let mut left = 0;
    let mut right = l;
    let mut middle = (left + right) / 2;

    loop {
        // println!("middle:{}", middle);

        let mut yokan_count = 0;
        let mut temp_length = 0;

        for value in p.iter() {
            temp_length += value;
            // println!("temp_length:{}", temp_length);

            if temp_length >= middle {
                yokan_count += 1;
                temp_length = 0;
                // println!("yokan_count:{}", yokan_count);
            }
        }

        if yokan_count >= k + 1 {
            // println!("success");
            // println!("{} {}", left, right);

            let old_middle = middle;

            left = middle;
            middle = (left + right) / 2;

            if middle == old_middle {
                break;
            }
        } else {
            // println!("failed");

            // 失敗
            right = middle;
            middle = (left + right) / 2;
        }
    }

    println!("{}", middle);
}
