use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: isize,
        n: usize,
        ab: [(usize, usize); n],
    }

    MyStruct { h, ab }.main();
}

struct MyStruct {
    h: isize,
    // n: usize,
    ab: Vec<(usize, usize)>,
}

impl MyStruct {
    fn main(&mut self) {
        let mut memo = vec![-1; 10usize.pow(5)];

        let result = self.dfs(self.h, &mut memo);

        println!("{}", result);
    }

    fn dfs(&self, rest_h: isize, memo: &mut Vec<isize>) -> usize {
        // 終了条件
        if rest_h <= 0 {
            return 0;
        }

        if memo[rest_h as usize] != -1 {
            return memo[rest_h as usize] as usize;
        }

        // 次に進む
        let mut min = std::usize::MAX;

        for (a, b) in &self.ab {
            let maigc_point = self.dfs(rest_h - *a as isize, memo) + b;
            min = std::cmp::min(min, maigc_point);
        }

        memo[rest_h as usize] = min as isize;

        min
    }
}
