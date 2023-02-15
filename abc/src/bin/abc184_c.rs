use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    if (a, b) == (c, d) {
        println!("0");
        return;
    }

    // 1 手
    if one_hand(a, b, c, d) {
        println!("1");
        return;
    }

    // 2 手

    // 5 * 5 の 25マスは1手で移動できるので、
    // さらにそこから1手で移動できるか
    for i in -2..=2 {
        for j in -2..=2 {
            if one_hand(a + i, b + j, c, d) {
                println!("2");
                return;
            }
        }
    }

    // 縦に3マス、横に3マスは1手で移動できるので、
    // さらにそこから1手で移動できるか
    for (i, j) in vec![(3, 0), (-3, 0), (0, 3), (0, -3)] {
        if one_hand(a + i, b + j, c, d) {
            println!("2");
            return;
        }
    }

    // 斜め2回で移動できるか
    if ((a - c).abs() + (b - d).abs()) % 2 == 0 {
        println!("2");
        return;
    }

    println!("3");
}

fn one_hand(a: isize, b: isize, c: isize, d: isize) -> bool {
    if (a - c).abs() + (b - d).abs() <= 3 {
        return true;
    }

    if a + b == c + d {
        return true;
    }

    if a - b == c - d {
        return true;
    }

    false
}
