use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
    }

    let mut strength = x;
    let mut exp = 0;

    while strength * a - strength < b {
        if strength * a >= y {
            println!("{}", exp);
            return;
        }

        exp += 1;
        strength *= a;
    }

    exp += (y - 1 - strength) / b;

    println!("{}", exp);
}
