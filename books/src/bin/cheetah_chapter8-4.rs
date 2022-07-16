// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 第8章 自動車ローン
///
fn main() {
    input! {
        price: f64,
        monthly_payment: f64,
        loan_term: i32,
    }

    let mut high = 100.;
    let mut low = 0.;
    let mut middle = 0.;

    while 1e-11 < high - low && 1e-11 < (high - low) / high {
        let mut balance = price;
        middle = (high + low) / 2.;

        for _ in 0..loan_term {
            balance *= middle / 1200. + 1.;
            balance -= monthly_payment;
        }

        if 0. < balance {
            high = middle;
        } else {
            low = middle;
        }
    }

    println!("{}", middle);
}
