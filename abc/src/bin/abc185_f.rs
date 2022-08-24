use proconio::input;

fn main() {
    input! {
        n :usize,
        q: usize,
        a: [usize; n],
        t_x_y: [(usize, usize, usize); q]
    }

    let mut tree = SegmentTreeRmq::new(n);

    for i in 0..n {
        tree.update(i, a[i] as isize);
    }

    for (t, x, y) in t_x_y {
        if t == 1 {
            let value = tree.query(x - 1, x);

            tree.update(x - 1, value ^ y as isize);
        } else {
            println!("{}", tree.query(x - 1, y));
        }
    }
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
            data[i] = 0;
        }

        SegmentTreeRmq { n, data }
    }

    fn update(&mut self, k: usize, a: isize) {
        let mut i = k + self.n - 1;
        self.data[i] = a;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1] ^ self.data[i * 2 + 2];
        }
    }

    ///
    /// [a, b) のxorを求める
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

        v_l ^ v_r
    }
}
