use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Puzzle {
    // list: [usize; 9],
    p: Vec<usize>,
    space: usize,
    step: usize,
}

fn main() {
    input! {
        m: usize,
        u_v: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }

    let mut list = vec![Vec::new(); 9];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let space = (0..9).filter(|x| !p.contains(x)).next().unwrap();

    let mut queue = VecDeque::new();

    let init = Puzzle { p, space, step: 0 };

    queue.push_back(init);

    let mut visited = HashSet::new();

    while let Some(current) = queue.pop_front() {
        // 終了条件
        if is_target(&current) {
            println!("{}", current.step);
            return;
        }

        if visited.contains(&current.p) {
            continue;
        }

        visited.insert(current.p.clone());

        for &next_vertex in list[current.space].iter() {
            let mut piece = 100;

            for i in 0..9 {
                if current.p[i] == next_vertex {
                    piece = i;
                    break;
                }
            }

            if piece == 100 {
                panic!("not found piece");
            }

            let space_vertex = current.space;

            let mut next_puzzle = current.clone();

            next_puzzle.p[piece] = space_vertex;
            next_puzzle.space = next_vertex;
            next_puzzle.step += 1;

            if visited.contains(&next_puzzle.p) {
                continue;
            }

            queue.push_back(next_puzzle);
        }
    }

    println!("-1");
}

fn is_target(current: &Puzzle) -> bool {
    for (i, &v) in current.p.iter().enumerate() {
        if i != v {
            return false;
        }
    }

    true
}
