use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x_y: [(isize, isize); n],
    }

    let mut result = 0;

    x_y.sort_by_key(|&(x, _)| x);

    let x_median = x_y[x_y.len() / 2].0;

    result += x_y
        .iter()
        .map(|&(x, _)| (x_median - x).abs())
        .sum::<isize>();

    x_y.sort_by_key(|&(_, y)| y);

    let y_median = x_y[x_y.len() / 2].1;

    result += x_y
        .iter()
        .map(|&(_, y)| (y_median - y).abs())
        .sum::<isize>();

    println!("{}", result);
}
