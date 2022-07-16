use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort();

    let mut money = k;
    let mut current = 0;
    let mut prev = 0;

    for &(a, b) in ab.iter() {
        if money < a - prev {
            current += money;
            println!("{}", current);
            return;
        }

        money -= a - prev;
        money += b;
        prev = a;

        current = a;
    }

    current += money;

    println!("{}", current);
}
