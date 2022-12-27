use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut result = 0usize;

    let mut mod_0 = 0usize;

    for x in 1..=n {
        if x % k == 0 {
            mod_0 += 1;
        }
    }

    result += mod_0.pow(3);

    if k % 2 == 0 {
        let mut k_2 = 0usize;

        for x in 1..=n {
            if x % k == k / 2 {
                k_2 += 1;
            }
        }

        result += k_2.pow(3);
    }

    println!("{}", result);
}
