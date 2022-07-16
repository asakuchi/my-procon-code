use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64,
    }

    let distance = (x.powf(2.) + y.powf(2.)).sqrt();

    if distance < r {
        println!("2");
    } else {
        let result = (distance / r).ceil();

        println!("{}", result);
    }
}
