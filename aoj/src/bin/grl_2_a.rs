use std::io;

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

///
/// 最小全域木
///
fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let e: usize = iter.next().unwrap().parse().unwrap();

    let mut s_t_w: Vec<(usize, usize, usize)> = Vec::with_capacity(e);

    for _ in 0..e {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s: usize = iter.next().unwrap().parse().unwrap();
        let t: usize = iter.next().unwrap().parse().unwrap();
        let w: usize = iter.next().unwrap().parse().unwrap();

        s_t_w.push((s, t, w));
    }

    // ------------------------------------

    // 最小全域木の辺の重みの総和
    let mut total_cost = 0;

    // 重みの昇順にソート
    s_t_w.sort_by_key(|x| x.2);

    let mut set = DisjointSet::new_with(v as isize);

    for i in 0..e {
        if !set.same(s_t_w[i].0 as isize, s_t_w[i].1 as isize) {
            total_cost += s_t_w[i].2;
            set.unite(s_t_w[i].0 as isize, s_t_w[i].1 as isize);
        }
    }

    println!("{}", total_cost);
}
