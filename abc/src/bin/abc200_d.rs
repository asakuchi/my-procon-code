use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = HashMap::new();
    let mut list_1 = Vec::new();

    if rec(n, &a, 0, 0, &mut dp, &mut list_1) {
        println!("Yes");

        let sum: usize = list_1.iter().map(|&i| a[i]).sum();

        let list_2 = dp.get(&(sum % 200)).unwrap();

        let text_1 = list_1.iter().map(|&i| i + 1).format(" ");
        let text_2 = list_2.iter().map(|&i| i + 1).format(" ");

        println!("{} {}", list_1.len(), text_1);
        println!("{} {}", list_2.len(), text_2);
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    a: &Vec<usize>,
    index: usize,
    remain: usize,
    dp: &mut HashMap<usize, Vec<usize>>,
    list_1: &mut Vec<usize>,
) -> bool {
    // println!("index {} rem {} list {:?}", index, remain, list_1);

    if let Some(list_2) = dp.get(&remain) {
        if list_1.len() > 0 && list_1 != list_2 {
            return true;
        }
    }

    if n != index {
        // 使う
        list_1.push(index);

        if rec(n, a, index + 1, (remain + a[index]) % 200, dp, list_1) {
            return true;
        }

        list_1.pop();

        // 使わない
        if rec(n, a, index + 1, remain, dp, list_1) {
            return true;
        }
    }

    dp.insert(remain, list_1.clone());

    false
}
