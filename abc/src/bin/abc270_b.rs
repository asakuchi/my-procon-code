use proconio::input;

fn main() {
    input! {
        mut x: isize,
        mut y: isize,
        mut z: isize,
    }

    if x < 0 {
        x *= -1;
        y *= -1;
        z *= -1;
    }

    if y < 0 {
        println!("{}", x);
        return;
    }

    // x, y は正
    if x < y {
        println!("{}", x);
        return;
    }

    // y < x

    if z < 0 {
        println!("{}", x + z.abs() * 2);
        return;
    }

    // x,y,z は正

    if z < y {
        println!("{}", x);
        return;
    }

    println!("-1");
}
