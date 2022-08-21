//!
//! Range Sum Query(RSQ)
//! セグメント木
//!
fn main() {
    let n = 8;

    let mut tree = SegmentTree::new(n);

    tree.add(0, 10);
    tree.add(1, 20);
    tree.add(2, 30);
    tree.add(3, 40);
    tree.add(4, 50);
    tree.add(5, 60);
    tree.add(6, 70);
    tree.add(7, 80);

    println!("{:?}", &tree.data[0..n * 2 - 1]);

    assert_eq!(tree.query(0, 1), 10);
    assert_eq!(tree.query(1, 2), 20);
    assert_eq!(tree.query(2, 3), 30);
    assert_eq!(tree.query(3, 4), 40);
    assert_eq!(tree.query(4, 5), 50);
    assert_eq!(tree.query(5, 6), 60);
    assert_eq!(tree.query(6, 7), 70);
    assert_eq!(tree.query(7, 8), 80);

    assert_eq!(tree.query(0, 2), 30);
    assert_eq!(tree.query(2, 4), 70);
    assert_eq!(tree.query(4, 6), 110);
    assert_eq!(tree.query(6, 8), 150);

    assert_eq!(tree.query(0, 4), 100);
    assert_eq!(tree.query(4, 8), 260);

    assert_eq!(tree.query(0, 8), 360);

    assert_eq!(tree.query(1, 4), 90);
    assert_eq!(tree.query(3, 6), 150);
    assert_eq!(tree.query(1, 7), 270);
}

struct SegmentTree {
    n: usize,
    data: Vec<isize>,
}

impl SegmentTree {
    fn new(size: usize) -> SegmentTree {
        let mut data = vec![0; 1 << 20];

        let mut n = 1;

        while n < size {
            n *= 2;
        }

        for i in 0..2 * n - 1 {
            data[i] = 0;
        }

        SegmentTree { n, data }
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
    fn add(&mut self, k: usize, a: isize) {
        let mut i = k + self.n - 1;
        self.data[i] += a;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1] + self.data[i * 2 + 2];
        }
    }

    ///
    /// [a, b) の合計を求める
    ///
    fn query(&self, a: usize, b: usize) -> isize {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if r <= a || b <= l {
            return 0;
        }

        if a <= l && r <= b {
            return self.data[k];
        }
        let v_l = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let v_r = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);

        v_l + v_r
    }
}
