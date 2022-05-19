use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    'a_loop: for a in 1..n {
        for b in 1..n {
            if n <= a * b {
                continue 'a_loop;
            }

            result += 1;
        }
    }

    println!("{}", result);
}
