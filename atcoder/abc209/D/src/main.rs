use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        cd: [(Usize1, Usize1); q],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &ab {
        list[a].push(b);
        list[b].push(a);
    }

    let mut colors = vec![0; n];

    rec(0, 1, &list, &mut colors);

    for &(c, d) in &cd {
        if colors[c] == colors[d] {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}

fn rec(current: usize, current_color: usize, list: &Vec<Vec<usize>>, colors: &mut Vec<usize>) {
    colors[current] = current_color;

    for &next in &list[current] {
        if colors[next] != 0 {
            continue;
        }

        let next_color = if current_color == 1 { 2 } else { 1 };

        rec(next, next_color, list, colors);
    }
}
