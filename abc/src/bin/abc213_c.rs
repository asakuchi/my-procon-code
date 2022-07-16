use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut h_list = Vec::new();
    let mut w_list = Vec::new();

    for &(a, b) in ab.iter() {
        h_list.push(a);
        w_list.push(b);
    }

    h_list.sort();
    h_list.dedup();
    w_list.sort();
    w_list.dedup();

    let mut h_map = std::collections::HashMap::new();
    let mut w_map = std::collections::HashMap::new();

    for i in 0..h_list.len() {
        h_map.insert(h_list[i], i + 1);
    }

    for i in 0..w_list.len() {
        w_map.insert(w_list[i], i + 1);
    }

    for (a, b) in ab.iter() {
        println!("{} {}", h_map.get(a).unwrap(), w_map.get(b).unwrap(),);
    }
}
