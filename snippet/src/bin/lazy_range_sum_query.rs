//!
//! Range Sum Query and Range Add Query (RSQ and RAQ)
//! 遅延評価セグメント木
//!
fn main() {
    let n = 8;

    let mut _tree = LazySegmentTree::new(n);
}

struct LazySegmentTree {
    n: usize,
    data: Vec<isize>,
    lazy: Vec<isize>,
}

impl LazySegmentTree {
    fn new(size: usize) -> LazySegmentTree {
        let mut data = vec![0; 1 << 20];
        let lazy = vec![0; 1 << 20];

        let mut n = 1;

        while n < size {
            n *= 2;
        }

        for i in 0..2 * n - 1 {
            data[i] = 0;
        }

        LazySegmentTree { n, data, lazy }
    }

    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            self.data[k] += self.lazy[k];

            if r - l > 1 {
                self.lazy[2 * k + 1] += self.lazy[k] / 2;
                self.lazy[2 * k + 2] += self.lazy[k] / 2;
            }

            self.lazy[k] = 0;
        }
    }

    ///
    /// 値を設定する
    /// 初期化時にしか使わないはず
    ///
    fn _update(&mut self, k: usize, a: isize) {
        let mut i = k + self.n - 1;
        self.data[i] = a;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1] + self.data[i * 2 + 2];
        }
    }

    ///
    /// 加算
    ///
    fn add(&mut self, a: usize, b: usize, x: isize) {
        self.add_inner(a, b, x, 0, 0, self.n);
    }

    fn add_inner(&mut self, a: usize, b: usize, x: isize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);

        if b <= l || r <= a {
            return;
        }

        if a <= l && r <= b {
            self.lazy[k] += (r as isize - l as isize) * x;
            self.eval(k, l, r);
        } else {
            self.add_inner(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.add_inner(a, b, x, 2 * k + 2, (l + r) / 2, r);
            self.data[k] = self.data[2 * k + 1] + self.data[2 * k + 2];
        }
    }

    ///
    /// [a, b) の合計を求める
    ///
    fn query(&mut self, a: usize, b: usize) -> isize {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if r <= a || b <= l {
            return 0;
        }

        self.eval(k, l, r);

        if a <= l && r <= b {
            return self.data[k];
        }

        let v_l = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let v_r = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);

        v_l + v_r
    }
}
