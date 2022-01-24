// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![0; 2 * n]; 2 * n];

    for i in 0..2 * n {
        for j in i + 1..2 * n {
            input! { value: usize }
            a[i][j] = value;
        }
    }

    MyStruct { n, a }.main();
}

struct MyStruct {
    n: usize,
    a: Vec<Vec<usize>>,
}

impl MyStruct {
    fn main(&mut self) {
        let mut pairs = Vec::new();
        let mut selected = vec![false; 2 * self.n];

        let result = self.dfs(&mut pairs, &mut selected);

        println!("{}", result);
    }

    fn dfs(&mut self, pairs: &mut Vec<(usize, usize)>, selected: &mut Vec<bool>) -> usize {
        if pairs.len() == self.n {
            let mut result = 0;

            for (first, second) in pairs {
                result ^= self.a[*first][*second];
            }

            return result;
        }

        let mut first = 10000;

        for i in 0..2 * self.n {
            if !selected[i] {
                first = i;
                break;
            }
        }

        selected[first] = true;

        let mut result = 0;

        for i in 0..2 * self.n {
            if !selected[i] {
                pairs.push((first, i));
                selected[i] = true;

                result = std::cmp::max(result, self.dfs(pairs, selected));

                pairs.pop();
                selected[i] = false;
            }
        }

        selected[first] = false;

        result
    }
}
