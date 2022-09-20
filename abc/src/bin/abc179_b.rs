use proconio::input;

fn main() {
    input! {
        n: usize,
        dice: [(usize, usize); n],
    }

    let mut zoro_count = 0;

    for (first, second) in dice {
        if first == second {
            zoro_count += 1;
        } else {
            zoro_count = 0;
        }

        if zoro_count == 3 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
