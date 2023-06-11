// use rand::Rng;
use std::{cmp::Reverse, collections::HashSet};
// use std::collections::BinaryHeap;
// use std::collections::HashSet;

use num_integer::Roots;

use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    // let start = Instant::now();

    // let mut rng = rand::thread_rng();

    input! {
        n: usize,
        m: usize,
        k: usize,
        x_y: [(isize, isize); n],
        u_v_w: [(Usize1, Usize1, isize); m],
        a_b: [(isize, isize); k],
    }

    // 頂点通しをつなぐ 最小全域木

    let mut b = vec![0; m];
    b[0] = 1;

    let mut edge: Vec<_> = u_v_w.iter().enumerate().collect();

    // 重みの昇順にソート
    edge.sort_by_key(|&(_, (_, _, w))| w);

    let mut uf = UnionFind::new(n);

    for &(index, &(u, v, _w)) in &edge {
        if !uf.equiv(u, v) {
            uf.union(u, v);

            b[index] = 1;
        }
    }

    // ------------------------------------

    // 住人からみて一番近い頂点との距離を頂点の強度とする

    // let mut p = vec![5_000; n];

    let mut p = vec![0; n];

    for i in 0..k {
        let (a, b) = a_b[i];

        // 一番近い頂点を探す
        let mut near_vertex = 0;
        let mut min_distance = 5_000 * 5_000;

        for j in 0..n {
            let (x, y) = x_y[j];

            let distance = (a - x).pow(2) + (b - y).pow(2);

            if distance < min_distance {
                near_vertex = j;
                min_distance = distance;
            }
        }

        p[near_vertex] = p[near_vertex].max(min_distance);
    }

    // -------------------------------------------------

    // 無駄な強度が発生している可能性がある
    // 強度が強い順に連結していけば無駄な部分を省ける？？

    // let mut list = vec![Vec::new(); n+k];
    let mut edge_duplicated = Vec::new();

    for i in 0..n {
        let (x, y) = x_y[i];
        let power = p[i];

        for j in 0..k {
            let (a, b) = a_b[j];

            if (x - a).pow(2) + (y - b).pow(2) <= power * power {
                edge_duplicated.push((i, j, power));
            }
        }
    }

    edge_duplicated.sort_by_key(|&(_, _, w)| Reverse(w));

    let mut edge = Vec::new();
    let mut used_v = vec![false; n];

    for &(vertex, person, power) in &edge_duplicated {
        if !used_v[vertex] {
            used_v[vertex] = true;

            edge.push((vertex, person, power));
        }
    }

    let mut uf = UnionFind::new(n + k);

    // 頂点は既に連結
    for i in 0..n {
        uf.union(0, i);
    }

    let mut p_2 = vec![0; n];

    let mut rest_person = HashSet::new();

    for i in 0..k {
        rest_person.insert(i);
    }

    for &(vertex, _person, power) in &edge {
        let (x, y) = x_y[vertex];

        for &next_person in rest_person.clone().iter() {
            let (next_a, next_b) = a_b[next_person];

            if (x - next_a).pow(2) + (y - next_b).pow(2) <= power {
                if !uf.equiv(vertex, n + next_person) {
                    // println!(
                    //     "v {} {:?} p {} {:?} distance {} <= {} {}",
                    //     vertex,
                    //     (x, y),
                    //     n + next_person,
                    //     (next_a, next_b),
                    //     (x - next_a).pow(2) + (y - next_b).pow(2),
                    //     power,
                    //     uf.equiv(vertex, n + next_person)
                    // );

                    uf.union(vertex, n + next_person);

                    p_2[vertex] = p[vertex];

                    rest_person.remove(&next_person);
                }
            }
        }
    }

    // ----------------------------------------------------

    // 使っていない頂点をOFFにしたい
    let mut b = vec![0; m];
    b[0] = 1;

    let orig_edge: Vec<_> = u_v_w.iter().enumerate().collect();

    let mut edge = Vec::new();
    let mut worthless_edge = Vec::new();

    let mut need_vertex = HashSet::new();

    for i in 0..n {
        if p[i] != 0 {
            need_vertex.insert(i);
        }
    }

    for &(index, &(u, v, w)) in &orig_edge {
        // 使っていない頂点への辺は優先度を下げる
        if p[u] == 0 || p[v] == 0 {
            worthless_edge.push((index, (u, v, w)));
        } else {
            edge.push((index, (u, v, w)));
        }
    }

    // 重みの昇順にソート
    edge.sort_by_key(|&(_, (_, _, w))| w);
    worthless_edge.sort_by_key(|&(_, (_, _, w))| w);

    let mut uf = UnionFind::new(n);

    for &(index, (u, v, _w)) in &edge {
        if !uf.equiv(u, v) {
            uf.union(u, v);

            b[index] = 1;

            need_vertex.remove(&u);
            need_vertex.remove(&v);
        }
    }

    for &(index, (u, v, _w)) in &worthless_edge {
        if need_vertex.len() == 0 {
            // 必要な頂点が連結できるまで続ける
            break;
        }

        if !uf.equiv(u, v) {
            uf.union(u, v);

            b[index] = 1;

            need_vertex.remove(&u);
            need_vertex.remove(&v);
        }
    }

    // output(&p, &b);
    output(&p_2, &b);
}

fn score(p: &Vec<isize>, b: &Vec<isize>, m: usize, u_v_w: &Vec<(usize, usize, isize)>) -> isize {
    let mut result = 0;

    for &value in p.iter() {
        result += value * value;
    }

    for i in 0..m {
        if b[i] == 1 {
            result += u_v_w[i].2;
        }
    }

    result
}

fn output(p: &Vec<isize>, b: &Vec<usize>) {
    println!(
        "{}",
        p.iter()
            .map(|&x| if x > 0 { x.sqrt() + 1 } else { 0 })
            .format(" ")
    );
    println!("{}", b.iter().format(" "));
}
