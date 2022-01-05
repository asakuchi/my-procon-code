// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

///
/// 第7章 会社の組織と給与
///
fn main() {
    input! {
        n: usize,
        relations: [Chars; n],
    }

    let mut sum = 0;

    let mut memo = vec![-1; n];

    for i in 0..n {
        sum += salary(&relations, i, &mut memo);
    }

    println!("{:?}", sum);
}

fn salary(relations: &Vec<Vec<char>>, i: usize, memo: &mut Vec<isize>) -> isize {
    if memo[i] != -1 {
        return memo[i];
    }

    if !relations[i].contains(&'Y') {
        memo[i] = 1;
        return 1;
    }

    let mut sum = 0;

    for (j, &relation) in relations[i].iter().enumerate() {
        if i == j {
            continue;
        }

        if relation == 'Y' {
            sum += salary(relations, j, memo);
        }
    }

    memo[i] = sum;

    sum
}
