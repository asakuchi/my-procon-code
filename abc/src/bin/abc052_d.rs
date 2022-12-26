use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        x: [usize; n],
    }

    let mut result = 0;

    let mut town = 0;

    while town != n - 1 {
        let score_a = (x[town + 1] - x[town]) * a;

        result += score_a.min(b);

        town += 1;
    }

    println!("{}", result);
}
