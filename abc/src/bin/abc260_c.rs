use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    if n == 1 {
        println!("0");
        return;
    }

    println!("{}", red(n, 1, x, y));
}

fn red(level: usize, count: usize, x: usize, y: usize) -> usize {
    if level == 1 {
        return 0;
    }

    red(level - 1, count, x, y) + blue(level, count * x, x, y)
}

fn blue(level: usize, count: usize, x: usize, y: usize) -> usize {
    if level == 1 {
        return count;
    }

    red(level - 1, count, x, y) + blue(level - 1, count * y, x, y)
}
