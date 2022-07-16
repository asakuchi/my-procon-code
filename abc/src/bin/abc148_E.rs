use proconio::input;

fn main() {
    input! {
        mut n: isize,
    }

    if n % 2 != 0 {
        println!("0");
        return;
    }

    let result = g2(n, 2).min(g2(n, 5));

    println!("{}", result);
}

fn g1(n: isize, p: isize) -> isize {
    if n == 0 {
        return 0;
    }

    n / p + g1(n / p, p)
}

fn g2(n: isize, p: isize) -> isize {
    let mut result = g1(n / 2, p);

    if p == 2 {
        result += n / 2;
    }

    result
}
