use proconio::input;

fn main() {
    input! {
        x_y: [(isize, isize); 4],
    }

    let mut ok = true;

    for i in 0..4 {
        let p_1 = x_y[(i + 4 - 1) % 4];
        let p_2 = x_y[(i + 4) % 4];
        let p_3 = x_y[(i + 4 + 1) % 4];

        let a = (p_1.0 - p_2.0, p_1.1 - p_2.1);
        let b = (p_3.0 - p_2.0, p_3.1 - p_2.1);

        let gai = a.0 * b.1 - a.1 * b.0;

        if gai >= 0 {
            ok = false;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
