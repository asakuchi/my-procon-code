use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut town: Vec<_> = xy
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| (x, y, i))
        .collect();

    let mut loads = Vec::new();

    town.sort_by_key(|t| t.0);

    for i in 1..n {
        let (x, _y, index) = town[i];
        let (prev_x, _prev_y, prev_index) = town[i - 1];

        loads.push((prev_index, index, (x - prev_x).abs()));
    }

    town.sort_by_key(|t| t.1);

    for i in 1..n {
        let (_x, y, index) = town[i];
        let (_prev_x, prev_y, prev_index) = town[i - 1];

        loads.push((prev_index, index, (y - prev_y).abs()));
    }

    loads.sort_by_key(|x| x.2);

    let mut set = UnionFind::new(n);

    let mut result = 0;

    for &(a, b, cost) in &loads {
        if !set.equiv(a, b) {
            set.union(a, b);
            result += cost;
        }
    }

    println!("{}", result);
}
