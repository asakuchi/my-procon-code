//!
//! Range Minimum Query(RMQ)
//! セグメント木
//!
//! https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A&lang=ja
//!

fn input_nq() -> (usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    (n, q)
}

fn input_com_xy() -> (usize, usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let com: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();
    let y: usize = iter.next().unwrap().parse().unwrap();

    (com, x, y)
}
fn main() {
    let (n, q) = input_nq();

    let mut tree = SegmentTreeRmq::new(n);

    for i in 0..n {
        // 2^31 - 1 で初期化
        tree.update(i, std::i32::MAX as isize);
    }

    for _ in 0..q {
        let (com, x, y) = input_com_xy();

        match com {
            0 => {
                tree.update(x, y as isize);
            }
            _ => {
                // 入力は閉区間で渡されるので +1
                println!("{}", tree.query(x, y + 1));
            }
        }
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
            data[i] = std::isize::MAX;
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
