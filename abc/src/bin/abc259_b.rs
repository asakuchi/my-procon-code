use procon_library_rs::geometry::angle::{Degree, ToRadian};
use procon_library_rs::geometry::point2f::Point2f;
use proconio::input;

fn main() {
    input! {
        a_b: Point2f,
        d: Degree,
    }

    println!("{}", a_b.rotate(Point2f { x: 0., y: 0. }, d.to_radian()));
}
