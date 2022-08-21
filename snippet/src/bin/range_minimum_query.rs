//!
//! Range Minimum Query(RSM)
//! セグメント木
//!
fn main() {
    let mut tree = SegmentTree::new(8);

    tree.update(0, 3);
    tree.update(1, 5);
    tree.update(2, 2);
    tree.update(3, 11);
    tree.update(4, 9);
    tree.update(5, 6);
    tree.update(6, 20);
    tree.update(7, 8);

    assert_eq!(tree.query(0, 1), 3);
    assert_eq!(tree.query(1, 2), 5);
    assert_eq!(tree.query(2, 3), 2);
    assert_eq!(tree.query(3, 4), 11);
    assert_eq!(tree.query(4, 5), 9);
    assert_eq!(tree.query(5, 6), 6);
    assert_eq!(tree.query(6, 7), 20);
    assert_eq!(tree.query(7, 8), 8);

    assert_eq!(tree.query(0, 2), 3);
    assert_eq!(tree.query(2, 4), 2);
    assert_eq!(tree.query(4, 6), 6);
    assert_eq!(tree.query(6, 8), 8);

    assert_eq!(tree.query(0, 4), 2);
    assert_eq!(tree.query(4, 8), 6);

    assert_eq!(tree.query(0, 8), 2);
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
            data[i] = std::isize::MAX;
        }

        SegmentTree { n, data }
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
