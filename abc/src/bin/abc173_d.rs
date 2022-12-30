use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut i = 0;

    let mut result = 0;

    result += a.pop().unwrap();
    i += 1;

    while i < n - 1 {
        let x = a.pop().unwrap();
        result += x;
        i += 1;

        if i < n - 1 {
            result += x;
            i += 1;
        }
    }

    println!("{}", result);
}
