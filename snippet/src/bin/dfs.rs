fn main() {
    let n = 100;
    let mut a = vec![vec![0; n]; n];

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
        // 終了条件
        if pairs.len() == self.n {
            let mut result = 0;

            return result;
        }

        // 次に進む

        for i in 0..self.n {
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
