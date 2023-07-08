use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ok = vec![(1, 2), (2, 3), (4, 5), (5, 6), (7, 8), (8, 9)];

    if ok.contains(&(a, b)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
