use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edge: [(Usize1, Usize1); m],
    }

    let mut list = vec![vec![]; n];

    for &(a, b) in edge.iter() {
        list[a].push(b);
        list[b].push(a);
    }

    let mut result = 0;

    for i in 0..n {
        let count = list[i].iter().filter(|&x| *x < i).count();

        if count == 1 {
            result += 1;
        }
    }

    println!("{}", result);
}
