//!
//! Binary Indexed Tree (BIT)
//!
fn main() {
    let mut tree = BinaryIndexedTree::new(6);

    tree.add(1, 10);
    tree.add(2, 20);
    tree.add(3, 30);
    tree.add(4, 40);
    tree.add(5, 50);

    assert_eq!(tree.sum(1), 10);
    assert_eq!(tree.sum(2), 30);
    assert_eq!(tree.sum(3), 60);
    assert_eq!(tree.sum(4), 100);
    assert_eq!(tree.sum(5), 150);
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
