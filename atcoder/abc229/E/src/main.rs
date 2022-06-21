use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &ab {
        list[a].push(b);
        list[b].push(a);
    }

    let mut result = Vec::new();
    result.push(0);

    let mut uf = UnionFind::new(n);

    let mut size = 0;

    // n-1回
    for u in (1..n).rev() {
        size += 1;

        for &v in list[u].iter() {
            if v < u {
                continue;
            }

            if !uf.equiv(u, v) {
                uf.union(u, v);
                size -= 1;
            }
        }

        result.push(size);
    }

    result.reverse();

    for num in result {
        println!("{}", num);
    }
}
