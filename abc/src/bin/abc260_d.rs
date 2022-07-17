use proconio::fastout;
use proconio::input;
use proconio::marker::Isize1;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Isize1; n],
    }

    let mut ba: BTreeSet<isize> = BTreeSet::new();

    let mut uf = DisjointSet::new_with(n as isize);

    let mut size = vec![0; n];
    let mut turn_list = vec![0; n];

    for i in 0..n {
        let number = p[i];

        let mut next_num = 3_000_000;

        if let Some(next) = ba.range(number..).next() {
            next_num = next.clone();
        }

        if next_num != 3_000_000 {
            ba.remove(&next_num);
            ba.insert(number);

            let tmp = size[uf.find_set(next_num) as usize];

            uf.unite(next_num, number);
            size[uf.find_set(next_num) as usize] = tmp + 1;
        } else {
            ba.insert(number);
            size[uf.find_set(number) as usize] = 1;
        }

        if size[uf.find_set(number) as usize] == k {
            ba.remove(&number);
            turn_list[uf.find_set(number) as usize] = i + 1;
        }
    }

    let mut eaten_turn = vec![-1; n];

    for i in 0..n {
        if turn_list[uf.find_set(i as isize) as usize] != 0 {
            eaten_turn[i] = turn_list[uf.find_set(i as isize) as usize] as isize;
        }
    }

    for i in 0..n {
        println!("{}", eaten_turn[i]);
    }
}

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
