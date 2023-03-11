use proconio::input;

fn main() {
    input! {
        a: u128,
        x: u128,
        m: u128,
    }

    if a == 1 {
        println!("{}", x % m);

        return;
    }

    // let result = (1 - a.pow(x)) / (1 - a) % m;
    let result = (mod_pow(a, x, m * (a - 1)) - 1) / (a - 1);

    println!("{}", result);
}

fn mod_pow(x: u128, a: u128, modulo: u128) -> u128 {
    if a == 1 {
        return x % modulo;
    }

    if a % 2 == 1 {
        return (x * mod_pow(x, a - 1, modulo)) % modulo;
    }

    let t = mod_pow(x, a / 2, modulo);

    return (t * t) % modulo;
}
