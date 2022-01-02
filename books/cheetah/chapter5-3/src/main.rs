// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 解説見た後のコード
///
fn main() {
    input! {
        base: usize,
    }

    let mut list = Vec::new();

    'outer: for n in 2..base {
        for hundreds in 0..base {
            for tens in 0..base {
                for ones in 0..base {
                    if (hundreds * base.pow(2) + tens * base.pow(1) + ones * base.pow(0)) % n == 0
                        && (hundreds + tens + ones) % n != 0
                    {
                        // n の倍数 だが、各桁の和がnの倍数でないなら除外
                        continue 'outer;
                    }
                }
            }
        }

        list.push(n);
    }

    println!("{:?}", list);
}

///
/// 解説見る前のコード
///
#[allow(dead_code)]
fn main_ver1() {
    input! {
        base: usize,
    }

    let mut list = Vec::new();

    'outer: for b in 2..=base {
        // baseは30までなので、30進法の3桁まで確認すれば十分
        for number in (b..30usize.pow(3)).step_by(b) {
            let mut digit = number;
            let mut sum = digit % base;

            while digit / base > 0 {
                digit /= base;
                sum += digit % base;
            }

            if sum % b != 0 {
                continue 'outer;
            }
        }

        list.push(b);
    }

    println!("{:?}", list);
}
