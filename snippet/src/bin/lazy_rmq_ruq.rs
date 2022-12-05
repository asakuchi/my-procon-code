//!
//! Range Minimum Query and Range Update Query (RMQ and RUQ)
//! 遅延評価セグメント木
//!
//! https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_F&lang=ja
//!

fn input_tuple() -> (usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    (n, m)
}

fn input_tuple_4() -> (usize, usize, usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let o: usize = iter.next().unwrap().parse().unwrap();
    let p: usize = iter.next().unwrap_or("0").parse().unwrap();

    (n, m, o, p)
}

fn main() {
    let (n, q) = input_tuple();

    let mut tree = LazySegmentTreeRmq::new(n);

    for i in 0..n {
        tree.update(i, i + 1, std::i32::MAX as isize);
    }

    for _ in 0..q {
        let (com, s, t, x) = input_tuple_4();

        match com {
            0 => {
                tree.update(s, t + 1, x as isize);
            }
            _ => {
                println!("{}", tree.query(s, t + 1));
            }
        }
    }
}

///
///
/// Range Minimum Query and Range Update Query (RMQ and RUQ)
///
/// 遅延評価セグメント木
///
///
struct LazySegmentTreeRmq {
    n: usize,
    data: Vec<isize>,
    lazy: Vec<isize>,
    lazy_flag: Vec<bool>,
}

impl LazySegmentTreeRmq {
    fn new(size: usize) -> LazySegmentTreeRmq {
        let mut n = 1;

        while n < size {
            n *= 2;
        }

        let mut data = vec![0; 2 * n - 1];
        let lazy = vec![0; 2 * n - 1];
        let lazy_flag = vec![false; 2 * n - 1];

        for i in 0..2 * n - 1 {
            data[i] = std::isize::MAX;
        }

        LazySegmentTreeRmq {
            n,
            data,
            lazy,
            lazy_flag,
        }
    }

    fn lazy_evaluate(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy_flag[k] {
            self.data[k] = self.lazy[k];
            if r > 1 + l {
                self.lazy[k * 2 + 1] = self.lazy[k];
                self.lazy[k * 2 + 2] = self.lazy[k];
                self.lazy_flag[k * 2 + 1] = true;
                self.lazy_flag[k * 2 + 2] = true;
            }
            self.lazy_flag[k] = false;
        }
    }

    fn update(&mut self, a: usize, b: usize, x: isize) {
        self.update_inner(a, b, x, 0, 0, self.n);
    }

    fn update_inner(&mut self, a: usize, b: usize, x: isize, k: usize, l: usize, r: usize) {
        self.lazy_evaluate(k, l, r);

        if b <= l || r <= a {
            return;
        }

        if a <= l && r <= b {
            self.lazy[k] = x;
            self.lazy_flag[k] = true;
            self.lazy_evaluate(k, l, r);
        } else {
            self.update_inner(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.update_inner(a, b, x, 2 * k + 2, (l + r) / 2, r);
            self.data[k] = self.data[2 * k + 1].min(self.data[2 * k + 2]);
        }
    }

    ///
    /// [a, b) の最小値を求める
    ///
    fn query(&mut self, a: usize, b: usize) -> isize {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        self.lazy_evaluate(k, l, r);

        if r <= a || b <= l {
            return std::isize::MAX;
        }

        if a <= l && r <= b {
            return self.data[k];
        }

        let v_l = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let v_r = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);

        v_l.min(v_r)
    }
}
