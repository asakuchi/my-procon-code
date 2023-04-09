use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let mut count = 0usize;

    while a != b {
        let (new_a, new_b) = rec(a, b, &mut count);
        a = new_a;
        b = new_b;
    }

    println!("{}", count);
}

fn rec(a: usize, b: usize, count: &mut usize) -> (usize, usize) {
    let (a, b) = if a > b { (a, b) } else { (b, a) };

    let diff = a - b;

    let size = diff / b;

    if size > 0 {
        *count += size;

        return (a - size * b, b);
    }

    *count += 1;
    (a - b, b)
}
