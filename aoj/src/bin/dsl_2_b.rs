fn main() {
    let (n, q) = input_tuple();

    let mut tree = SegmentTree::new(n);

    for _ in 0..q {
        let (com, x, y) = input_tuple_3();

        match com {
            0 => {
                // x は 1-index
                tree.add(x - 1, y as isize);
            }
            _ => {
                println!("{}", tree.query(x - 1, y));
            }
        }
    }
}

struct SegmentTree {
    n: usize,
    data: Vec<isize>,
}

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

fn input_tuple_3() -> (usize, usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();

    (n, m, l)
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

    fn update(&mut self, k: usize, a: isize) {
        let mut i = k + self.n - 1;
        self.data[i] = a;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1] + self.data[i * 2 + 2];
        }
    }

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
