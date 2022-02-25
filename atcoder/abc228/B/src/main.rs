use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [Usize1; n],
    }

    let mut visited = vec![false; n];
    let mut next = x - 1;
    let mut result = 0;

    loop {
        if visited[next] {
            break;
        }

        visited[next] = true;
        result += 1;

        next = a[next];
    }

    println!("{}", result);
}
