use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        u_v: [(Usize1, Usize1); n],
        q: usize,
        x_y: [(Usize1, Usize1); q],
    }

    let mut list = vec![Vec::new(); n];
    let mut cycle = vec![HashSet::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);

        cycle[u].insert(v);
        cycle[v].insert(u);
    }

    loop {
        let mut changed = false;

        for i in 0..n {
            if cycle[i].len() == 1 {
                remove_edge(&mut cycle, i, i);

                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    let mut roots = HashSet::new();

    for i in 0..n {
        if cycle[i].len() > 1 {
            roots.insert(i);
        }
    }

    let mut belonging: Vec<_> = (0..n).collect();

    for &root in &roots {
        rec(&list, &roots, &mut belonging, root, root, root);
    }

    for &(x, y) in &x_y {
        if belonging[x] == belonging[y] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn remove_edge(cycle: &mut Vec<HashSet<usize>>, current: usize, parent: usize) {
    if cycle[current].len() == 1 {
        let &next = cycle[current].iter().next().unwrap();
        cycle[current].remove(&next);
        cycle[next].remove(&current);

        remove_edge(cycle, next, current);
    }
}

fn rec(
    list: &Vec<Vec<usize>>,
    roots: &HashSet<usize>,
    belonging: &mut Vec<usize>,
    root: usize,
    currnt: usize,
    parent: usize,
) {
    belonging[currnt] = root;

    for &next in list[currnt].iter() {
        if roots.contains(&next) {
            continue;
        }

        if next == parent {
            continue;
        }

        rec(list, roots, belonging, root, next, currnt);
    }
}
