use proconio::fastout;
use proconio::input;

#[derive(Debug)]
pub struct DisjointSet {
    rank: Vec<isize>,
    p: Vec<isize>,
}

impl DisjointSet {
    pub fn new() -> DisjointSet {
        DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
        }
    }

    pub fn new_with(size: isize) -> DisjointSet {
        let mut set = DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
        };

        for i in 0..size {
            set.make_set(i);
        }

        set
    }

    pub fn make_set(&mut self, x: isize) {
        self.p[x as usize] = x;
        self.rank[x as usize] = 0;
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
            self.p[b as usize] = a;
        } else {
            self.p[a as usize] = b;
            if self.rank[a as usize] == self.rank[b as usize] {
                self.rank[b as usize] += 1;
            }
        }
    }

    pub fn same(&mut self, x: isize, y: isize) -> bool {
        self.find_set(x) == self.find_set(y)
    }
}

#[fastout]
fn main() {
    input! {
        n: isize,
        m: usize,
        mut ab_org: [(isize, isize); m],
    }

    let ab: Vec<_> = ab_org.into_iter().map(|(a, b)| (a - 1, b - 1)).collect();

    let mut result = 0;

    for i in 0..m {
        let mut set = DisjointSet::new_with(n);

        for (j, (a, b)) in ab.iter().enumerate() {
            if i == j {
                continue;
            }

            set.unite(*a, *b);
        }

        if !set.same(ab[i].0, ab[i].1) {
            result += 1;
        }
    }

    println!("{}", result);
}
