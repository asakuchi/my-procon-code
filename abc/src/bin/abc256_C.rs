use proconio::input;

fn main() {
    input! {
        h1: isize,
        h2: isize,
        h3: isize,
        w1: isize,
        w2: isize,
        w3: isize,
    }

    let mut result = 0;

    for num_00 in 1..=30 {
        for num_01 in 1..=30 {
            for num_10 in 1..=30 {
                for num_11 in 1..=30 {
                    let num_02 = h1 - num_00 - num_01;
                    let num_12 = h2 - num_10 - num_11;
                    let num_20 = w1 - num_00 - num_10;
                    let num_21 = w2 - num_01 - num_11;

                    if num_02 < 1 || num_12 < 1 || num_20 < 1 || num_21 < 1 {
                        continue;
                    }

                    let num_22 = h3 - num_20 - num_21;

                    if num_22 < 1 {
                        continue;
                    }

                    if w3 == num_02 + num_12 + num_22 {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("{}", result);
}
