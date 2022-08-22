use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a_b: [(Usize1, Usize1); n - 1],
        q: usize,
        t_e_x: [(usize, Usize1, usize); q],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut parents = vec![0; n];

    search_parent(&list, 0, 0, &mut parents);

    let mut c = vec![0; n];

    let mut accum = vec![0isize; n];

    for &(t, e, x) in &t_e_x {
        let (a, b) = a_b[e];

        match t {
            1 => {
                if parents[b] == a {
                    accum[b] -= x as isize;
                    accum[0] += x as isize;
                } else {
                    accum[a] += x as isize;
                }
            }
            _ => {
                if parents[a] == b {
                    accum[a] -= x as isize;
                    accum[0] += x as isize;
                } else {
                    accum[b] += x as isize;
                }
            }
        }
    }

    rec(&list, 0, 0, &accum, &mut c, 0);

    for i in 0..n {
        println!("{}", c[i]);
    }
}

fn search_parent(list: &Vec<Vec<usize>>, current: usize, parent: usize, parents: &mut Vec<usize>) {
    parents[current] = parent;

    for &next in list[current].iter() {
        if next == parent {
            continue;
        }

        search_parent(list, next, current, parents);
    }
}

fn rec(
    list: &Vec<Vec<usize>>,
    current: usize,
    parent: usize,
    accum: &Vec<isize>,
    c: &mut Vec<usize>,
    point: usize,
) {
    let point = (point as isize + accum[current]) as usize;

    c[current] += point;

    // println!("current {} {}", current, c[current]);

    for &next in list[current].iter() {
        if next == parent {
            continue;
        }

        rec(list, next, current, accum, c, point);
    }
}
