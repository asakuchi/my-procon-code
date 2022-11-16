use proconio::{input, marker::Usize1};

///
/// ABC167D
///
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }

    let mut doubling = Doubling::new(n, k, &a);

    doubling.preprocess();

    println!("{}", doubling.get(k, 0) + 1);
}

struct Doubling {
    n: usize,
    log_k: usize,
    /// doubling[k][i] : i番目から 2^k 進んだ町
    doubling: Vec<Vec<usize>>,
}

impl Doubling {
    ///
    /// 初期化
    ///
    /// * `n` - 点の数
    /// * `k` - 最終的に求めたい step
    /// * `one_step` - 1 step でどこに移動するか
    ///
    fn new(n: usize, k: usize, one_step: &Vec<usize>) -> Doubling {
        let mut log_k = 0;

        while 1 << log_k <= k {
            log_k += 1;
        }

        let mut doubling = vec![vec![0; n]; log_k + 1];

        doubling[0] = one_step.clone();

        Doubling { n, log_k, doubling }
    }

    ///
    /// 事前処理
    ///
    fn preprocess(&mut self) {
        for j in 0..self.log_k {
            for i in 0..self.n {
                self.doubling[j + 1][i] = self.doubling[j][self.doubling[j][i]];
            }
        }
    }

    ///
    /// start から k step 目の場所を求める
    ///
    fn get(&self, k: usize, start: usize) -> usize {
        let mut current = start;

        for i in (0..self.log_k).rev() {
            if k & (1 << i) > 0 {
                current = self.doubling[i][current];
            }
        }

        current
    }
}
