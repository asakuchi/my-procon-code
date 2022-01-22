// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut hash = std::collections::HashMap::new();

    for (a, b) in &ab {
        let count = hash.entry(a).or_insert(0);
        *count += 1;

        let count = hash.entry(b).or_insert(0);
        *count += 1;
    }

    let result_1 = hash.values().all(|&x| x < 3);

    let mut set = DisjointSet::new();

    for i in 1..=n {
        set.make_set(i as i64);
    }

    let mut result_2 = true;

    for (a, b) in &ab {
        if set.same(*a as i64, *b as i64) {
            result_2 = false;
            break;
        }

        set.unite(*a as i64, *b as i64);
    }

    println!("{}", if result_1 && result_2 { "Yes" } else { "No" });
}

#[derive(Debug)]
pub struct DisjointSet {
    rank: Vec<i64>,
    p: Vec<i64>,
}

impl DisjointSet {
    pub fn new() -> DisjointSet {
        DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
        }
    }

    pub fn new_with(size: i64) -> DisjointSet {
        let mut set = DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
        };

        for i in 0..size {
            set.make_set(i);
        }

        set
    }

    pub fn make_set(&mut self, x: i64) {
        self.p[x as usize] = x;
        self.rank[x as usize] = 0;
    }

    pub fn find_set(&mut self, x: i64) -> i64 {
        if x != self.p[x as usize] {
            self.p[x as usize] = self.find_set(self.p[x as usize]);
        }

        self.p[x as usize]
    }

    pub fn unite(&mut self, x: i64, y: i64) {
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

    pub fn same(&mut self, x: i64, y: i64) -> bool {
        self.find_set(x) == self.find_set(y)
    }
}
