use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut list = Vec::new();

    for s_2 in 1..=9 {
        for s_3 in 0..=9 {
            for s_4 in 0..=9 {
                for s_6 in 0..=9 {
                    for s_8 in 0..=9 {
                        for s_9 in 0..=9 {
                            let s_1 = s_2;
                            let s_5 = s_6;
                            let s_7 = s_9;

                            let x = s_1 * 10usize.pow(8)
                                + s_2 * 10usize.pow(7)
                                + s_3 * 10usize.pow(6)
                                + s_4 * 10usize.pow(5)
                                + s_5 * 10usize.pow(4)
                                + s_6 * 10usize.pow(3)
                                + s_7 * 10usize.pow(2)
                                + s_8 * 10usize.pow(1)
                                + s_9 * 10usize.pow(0);

                            list.push(x);
                        }
                    }
                }
            }
        }
    }

    list.sort();

    println!("{}", list[n - 1]);
}
