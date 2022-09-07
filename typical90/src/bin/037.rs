use proconio::input;

const INF: isize = 1_000_000_000_000_000_000;

fn main() {
    input! {
        w: usize,
        n: usize,
        l_r_v: [(usize, usize, isize); n],
    }

    // dp[何個目][香辛料の合計]
    let mut dp = vec![vec![-INF; w + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        let (l, r, v) = l_r_v[i - 1];

        let mut tree = SegmentTreeRmq::new(w + 1);

        for j in 0..=w {
            tree.update(j, dp[i - 1][j]);
        }

        for j in 0..=w {
            let score_1 = dp[i - 1][j];
            dp[i][j] = score_1;

            let left = if j >= r { j - r } else { 0 };
            let right = if j + 1 >= l { j + 1 - l } else { 0 };

            let score_2 = tree.query(left, right) + v;

            dp[i][j] = dp[i][j].max(score_2);
        }
    }

    if dp[n][w] > 0 {
        println!("{}", dp[n][w]);
    } else {
        println!("-1");
    }
}

///
///
/// Range Minimum Query(RMQ)
///
/// セグメント木
///
///
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
            data[i] = -INF;
        }

        SegmentTreeRmq { n, data }
    }

    fn update(&mut self, k: usize, a: isize) {
        let mut i = k + self.n - 1;
        self.data[i] = a;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = std::cmp::max(self.data[i * 2 + 1], self.data[i * 2 + 2]);
        }
    }

    ///
    /// [a, b) の最大値を求める
    ///
    fn query(&self, a: usize, b: usize) -> isize {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if r <= a || b <= l {
            return -INF;
        }

        if a <= l && r <= b {
            return self.data[k];
        }
        let v_l = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let v_r = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);

        v_l.max(v_r)
    }
}
