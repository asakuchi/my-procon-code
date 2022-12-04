use proconio::input;

use ac_library_rs::ModInt998244353;

use ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
    }

    // dp
    // マスiからサイコロを投げる回数の期待値
    let mut tree = LazySegmentTreeRsq::new(2 * n + 1);

    for i in (0..n - 1).rev() {
        let dp_ij = tree.query(i, i + a[i] + 1);
        tree.add(i, i + 1, (dp_ij + mint::new(a[i] + 1)) / mint::new(a[i]));
    }

    println!("{}", tree.query(0, 1));
}

///
///
/// Range Sum Query and Range Add Query (RSQ and RAQ)
///
/// 遅延評価セグメント木
///
///
struct LazySegmentTreeRsq {
    n: usize,
    data: Vec<mint>,
    lazy: Vec<mint>,
}

impl LazySegmentTreeRsq {
    fn new(size: usize) -> LazySegmentTreeRsq {
        let mut n = 1;

        while n < size {
            n *= 2;
        }

        let mut data = vec![mint::new(0); 2 * n - 1];
        let lazy = vec![mint::new(0); 2 * n - 1];

        for i in 0..2 * n - 1 {
            data[i] = mint::new(0);
        }

        LazySegmentTreeRsq { n, data, lazy }
    }

    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != mint::new(0) {
            self.data[k] += self.lazy[k];

            if r - l > 1 {
                let lazy_k = self.lazy[k];

                self.lazy[2 * k + 1] += lazy_k / mint::new(2);
                self.lazy[2 * k + 2] += lazy_k / mint::new(2);
            }

            self.lazy[k] = mint::new(0);
        }
    }

    ///
    /// 値を設定する
    /// 初期化時にしか使わないはず
    ///
    fn _update(&mut self, k: usize, a: mint) {
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
    fn add(&mut self, a: usize, b: usize, x: mint) {
        self.add_inner(a, b, x, 0, 0, self.n);
    }

    fn add_inner(&mut self, a: usize, b: usize, x: mint, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);

        if b <= l || r <= a {
            return;
        }

        if a <= l && r <= b {
            self.lazy[k] += mint::new(r as isize - l as isize) * x;
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
    fn query(&mut self, a: usize, b: usize) -> mint {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> mint {
        if r <= a || b <= l {
            return mint::new(0);
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
