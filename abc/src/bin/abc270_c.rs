use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
        u_v: [(Usize1, Usize1); n-1],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let mut path = Vec::new();

    rec(n, x, y, &list, x, x, &mut path);

    let result = path
        .iter()
        .map(|x| (x + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
}

fn rec(
    _n: usize,
    _x: usize,
    y: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    parent: usize,
    path: &mut Vec<usize>,
) -> bool {
    if current == y {
        path.push(current);

        return true;
    }

    for &next in list[current].iter() {
        if next == parent {
            continue;
        }

        path.push(current);

        let result = rec(_n, _x, y, list, next, current, path);

        if result {
            return true;
        }

        path.pop();
    }

    false
}
