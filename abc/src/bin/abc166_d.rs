use proconio::input;

fn main() {
    input! {
        x: isize,
    }

    if x == 1 {
        println!("1 0");
        return;
    }

    let a_list = vec![1];
    let b_list = vec![1, -1];

    for &base_a in &a_list {
        for &base_b in &b_list {
            let mut a = base_a;

            let mut a_5: isize = a * a * a * a * a;

            while a_5.abs() <= 1_000_000_000_000 {
                let mut b = base_b;
                let mut b_5: isize = b * b * b * b * b;

                while b_5.abs() <= 1_000_000_000_000 {
                    if a_5 - b_5 == x {
                        println!("{} {}", a, b);
                        return;
                    }

                    b += base_b;
                    b_5 = b * b * b * b * b;
                }

                a += base_a;
                a_5 = a * a * a * a * a;
            }
        }
    }
}
