use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        u_v_w: [(Usize1, Usize1, usize); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v, w) in &u_v_w {
        list[u].push((v, w));
        list[v].push((u, w));
    }

    let mut visited = vec![false; n];
    let mut colors = vec![100; n];

    visited[0] = true;

    rec(n, &list, 0, 1, &mut visited, &mut colors);

    for color in colors {
        println!("{}", color);
    }
}

fn rec(
    n: usize,
    list: &Vec<Vec<(usize, usize)>>,
    current: usize,
    current_color: usize,
    visited: &mut Vec<bool>,
    colors: &mut Vec<usize>,
) {
    colors[current] = current_color;

    let same_color = if current_color == 1 { 1 } else { 0 };
    let different_color = if current_color == 1 { 0 } else { 1 };

    for &(next, weight) in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        rec(
            n,
            list,
            next,
            if weight % 2 == 0 {
                same_color
            } else {
                different_color
            },
            visited,
            colors,
        );
    }
}
