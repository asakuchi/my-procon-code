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

///
/// 最小全域木
/// クラスカルのアルゴリズム
/// Kruskal's Algorithm
///
#[fastout]
fn main() {
    input! {
        n: usize, // 頂点数
        m: usize, // 辺の数
        k: usize,
        mut abc_org: [(usize, usize, usize); m], // 辺が結ぶ2点の頂点と重み
    }

    // ------------------------------------

    let mut abc: Vec<_> = abc_org
        .into_iter()
        .map(|(a, b, c)| (a - 1, b - 1, c))
        .collect();

    // 最小全域木の辺の重みの総和
    let mut total_cost = 0;

    let mut size = n;

    // 重みの昇順にソート
    abc.sort_by_key(|x| x.2);

    let mut set = DisjointSet::new_with(n as isize);

    for i in 0..m {
        if size == k {
            break;
        }

        if !set.same(abc[i].0 as isize, abc[i].1 as isize) {
            total_cost += abc[i].2;
            set.unite(abc[i].0 as isize, abc[i].1 as isize);

            size -= 1;
        }
    }

    println!("{}", total_cost);
}
