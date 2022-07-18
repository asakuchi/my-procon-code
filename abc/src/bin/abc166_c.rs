use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        mut ab: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in ab.iter() {
        list[a].push(b);
        list[b].push(a);
    }

    let mut result = 0;

    for i in 0..n {
        let mut ok = true;

        for &next in list[i].iter() {
            if h[i] <= h[next] {
                ok = false;
                break;
            }
        }

        if ok {
            result += 1;
        }
    }

    println!("{}", result);
}
