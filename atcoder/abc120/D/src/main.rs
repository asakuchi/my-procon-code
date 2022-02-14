use proconio::fastout;
use proconio::input;

#[derive(Debug)]
pub struct DisjointSet {
    rank: Vec<isize>,
    p: Vec<isize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new() -> DisjointSet {
        DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
            size: vec![0; 1_000_000],
        }
    }

    pub fn new_with(size: isize) -> DisjointSet {
        let mut set = DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
            size: vec![0; 1_000_000],
        };

        for i in 0..size {
            set.make_set(i);
        }

        set
    }

    pub fn make_set(&mut self, x: isize) {
        self.p[x as usize] = x;
        self.rank[x as usize] = 0;
        self.size[x as usize] = 1;
    }

    pub fn find_set(&mut self, x: isize) -> isize {
        if x != self.p[x as usize] {
            self.p[x as usize] = self.find_set(self.p[x as usize]);
        }

        self.p[x as usize]
    }

    pub fn unite(&mut self, x: isize, y: isize) {
        let a = self.find_set(self.p[x as usize]);
        let b = self.find_set(self.p[y as usize]);

        if self.rank[a as usize] > self.rank[b as usize] {
            self.size[a as usize] += self.size[b as usize];
            self.size[b as usize] = 0;
            self.p[b as usize] = a;
        } else {
            self.size[b as usize] += self.size[a as usize];
            self.size[a as usize] = 0;
            self.p[a as usize] = b;
            if self.rank[a as usize] == self.rank[b as usize] {
                self.rank[b as usize] += 1;
            }
        }
    }

    pub fn same(&mut self, x: isize, y: isize) -> bool {
        self.find_set(x) == self.find_set(y)
    }

    pub fn size(&mut self, x: isize) -> usize {
        let root = self.find_set(x);
        self.size[root as usize]
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab_org: [(isize, isize); m],
    }

    let ab: Vec<_> = ab_org.into_iter().map(|(a, b)| (a - 1, b - 1)).collect();

    let mut result = vec![0 as isize; m];
    result[m - 1] = (n * (n - 1) / 2) as isize;

    let mut set = DisjointSet::new_with(n as isize);

    for i in (1..m).rev() {
        let (a, b) = ab[i];

        if set.same(a, b) {
            result[i - 1] = result[i];
        } else {
            result[i - 1] = result[i] - (set.size(a) * set.size(b)) as isize;
            set.unite(a, b);
        }
    }

    for i in 0..m {
        println!("{}", result[i]);
    }
}
