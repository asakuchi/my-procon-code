// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
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
