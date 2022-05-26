use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
        mut tx: [(usize, isize); q],
    }

    let mut tree = BinaryIndexedTree::new(200_000);

    for i in 0..q {
        let (t, x) = tx[i];

        match t {
            1 => {
                tree.add(x as usize, 1);
            }
            _ => {
                let mut ok: isize = 200_000;
                let mut ng = 0;

                while (ok - ng).abs() > 1 {
                    let mid = (ok + ng) / 2;
                    let solve = || tree.sum(mid as usize) >= x;

                    if solve() {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }

                println!("{}", ok);
                tree.add(ok as usize, -1);
            }
        }
    }
}

struct BinaryIndexedTree {
    n: usize,
    bit: Vec<isize>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> BinaryIndexedTree {
        let bit = vec![0; n];

        BinaryIndexedTree { n, bit }
    }

    fn add(&mut self, i: usize, x: isize) {
        let mut index = i as isize;

        while (index as usize) < self.n {
            self.bit[index as usize] += x;

            index += index & -index;
        }
    }

    fn sum(&self, i: usize) -> isize {
        let mut s = 0;

        let mut index = i as isize;

        while index > 0 {
            s += self.bit[index as usize];

            index -= index & -index;
        }

        s
    }
}
