use proconio::input;

fn main() {
    input! {
        a: usize, // g
        b: usize, // g
        w: usize, // kg
    }

    let w = w * 1_000;

    let mut unsatisfiable = true;

    let mut min_count = 1_000_000;
    let mut max_count = 0;

    for i in 1..=1_000_000 {
        if (a * i) <= w && w <= (b * i) {
            unsatisfiable = false;

            min_count = min_count.min(i);
            max_count = max_count.max(i);
        }
    }

    if unsatisfiable {
        println!("UNSATISFIABLE");
        return;
    }

    println!("{} {}", min_count, max_count);
}
