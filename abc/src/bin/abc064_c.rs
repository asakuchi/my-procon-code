use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut colors = vec![false; 8];
    let mut over_3200 = 0;

    for x in a {
        if x >= 3200 {
            over_3200 += 1;
        } else {
            for i in 0..8 {
                if x >= i * 400 && x < (i + 1) * 400 {
                    colors[i] = true;
                }
            }
        }
    }

    let mut under_3200 = 0;

    for color in colors {
        if color {
            under_3200 += 1;
        }
    }

    let min_var = if under_3200 == 0 {
        // 3200越えは全て1色
        1
    } else {
        // 3200超えは全て既存の1色にするので数えない
        under_3200
    };

    let max_var = under_3200 + over_3200;

    // println!("{:?}", colors);
    println!("{} {}", min_var, max_var);
}
