use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let list: Vec<_> = (1..=n).collect();

    a.sort();

    if a == list {
        println!("Yes");
    } else {
        println!("No");
    }
}
