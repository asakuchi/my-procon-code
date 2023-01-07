use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }

        let mut result = 0;

        for value in a {
            if value % 2 == 1 {
                result += 1;
            }
        }

        println!("{}", result);
    }
}
