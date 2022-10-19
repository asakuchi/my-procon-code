use proconio::input;

fn main() {
    input! {
        l: f64,
    }

    println!("{}", (l / 3.).powf(3.));
}
