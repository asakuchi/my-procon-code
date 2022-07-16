use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        mut ab: [(usize, usize); n],
    }

    MyStruct { n, x, ab }.main();
}

struct MyStruct {
    n: usize,
    x: usize,
    ab: Vec<(usize, usize)>,
}

impl MyStruct {
    fn main(&mut self) {
        let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; 100_100]; self.n + 100];

        let result = self.dfs(0, 0, &mut memo);

        if result {
            println!("Yes");
        } else {
            println!("No");
        }
    }

    fn dfs(&mut self, index: usize, position: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        // println!("{} {}", index, position);

        if let Some(value) = memo[index][position] {
            return value;
        }

        // 終了条件
        if index == self.n {
            return position == self.x;
        }

        // 次に進む

        let result = self.dfs(index + 1, position + self.ab[index].0, memo)
            || self.dfs(index + 1, position + self.ab[index].1, memo);

        memo[index][position] = Some(result);

        result
    }
}
