use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();

    let k = 1_000_000;

    let mut one_step = vec![0; n];

    for i in 0..n {
        if s[i] == 'R' {
            one_step[i] = i + 1;
        } else {
            one_step[i] = i - 1;
        }
    }

    let mut doubling = Doubling::new(n, k, &one_step);

    doubling.preprocess();

    let mut result = vec![0; n];

    for i in 0..n {
        let k_step_position = doubling.get(k, i);
        result[k_step_position] += 1;
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}

struct Doubling {
    n: usize,
    log_k: usize,
    doubling: Vec<Vec<usize>>,
}

impl Doubling {
    fn new(n: usize, k: usize, one_step: &Vec<usize>) -> Doubling {
        let mut log_k = 0;

        while 1 << log_k <= k {
            log_k += 1;
        }

        let mut doubling = vec![vec![0; n]; log_k + 1];

        doubling[0] = one_step.clone();

        Doubling { n, log_k, doubling }
    }

    fn preprocess(&mut self) {
        for j in 0..self.log_k {
            for i in 0..self.n {
                self.doubling[j + 1][i] = self.doubling[j][self.doubling[j][i]];
            }
        }
    }

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
