use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; n - 1],
        x_y: [(Usize1, usize); m],
    }

    let mut list = vec![Vec::new(); n];

    for i in 0..n - 1 {
        let parent = p[i];
        let child = i + 1;

        list[parent].push(child);
    }

    let mut insurance = HashMap::new();

    for &(x, y) in &x_y {
        if let Some(&value) = insurance.get(&x) {
            if y > value {
                insurance.insert(x, y);
            }
        } else {
            insurance.insert(x, y);
        }
    }

    let mut result = 0;

    rec(n, &list, &insurance, 0, 0, &mut result);

    println!("{}", result);
}

fn rec(
    n: usize,
    list: &Vec<Vec<usize>>,
    insurance: &HashMap<usize, usize>,
    current: usize,
    rest: usize,
    result: &mut usize,
) {
    let mut rest = rest;

    if let Some(&value) = insurance.get(&current) {
        // 補償対象に自分自身を含めるため+1
        if value + 1 > rest {
            rest = value + 1;
        }
    }

    if rest > 0 {
        *result += 1;
    }

    for &next in list[current].iter() {
        rec(
            n,
            list,
            insurance,
            next,
            if rest > 1 { rest - 1 } else { 0 },
            result,
        );
    }
}
