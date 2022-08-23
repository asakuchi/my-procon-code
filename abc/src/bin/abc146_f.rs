use itertools::Itertools;
use proconio::{input, marker::Chars};

const INF: isize = 1_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }

    let mut tree = SegmentTreeRmq::new(n + 1 + m);

    tree.update(n, 0);

    for i in (0..n).rev() {
        if s[i] == '1' {
            continue;
        }

        let x = tree.query(i + 1, i + m + 1);

        tree.update(i, x + 1);
    }

    if tree.query(0, 1) == INF {
        println!("-1");
        return;
    }

    let mut result = Vec::new();

    let mut prev = 0;

    for i in 0..=n {
        if s[i] == '1' {
            continue;
        }

        if tree.query(i, i + 1) + 1 == tree.query(prev, prev + 1) {
            result.push(i - prev);
            prev = i;
        }
    }

    let text = result.iter().format(" ");
    println!("{}", text);
}

struct SegmentTreeRmq {
    n: usize,
    data: Vec<isize>,
}

impl SegmentTreeRmq {
    fn new(size: usize) -> SegmentTreeRmq {
        let mut n = 1;

        while n < size {
            n *= 2;
        }

        let mut data = vec![0; 2 * n - 1];

        for i in 0..2 * n - 1 {
            data[i] = INF;
        }

        SegmentTreeRmq { n, data }
    }

    fn update(&mut self, k: usize, a: isize) {
        let mut i = k + self.n - 1;
        self.data[i] = a;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = std::cmp::min(self.data[i * 2 + 1], self.data[i * 2 + 2]);
        }
    }

    ///
    /// [a, b) の最小値を求める
    ///
    fn query(&self, a: usize, b: usize) -> isize {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if r <= a || b <= l {
            return INF;
        }

        if a <= l && r <= b {
            return self.data[k];
        }
        let v_l = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let v_r = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);

        v_l.min(v_r)
    }
}
