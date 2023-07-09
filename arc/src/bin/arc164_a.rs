use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            mut n: usize,
            k: usize,
        }

        let mut bit = Vec::new();

        while n != 0 {
            bit.push(n % 3);

            n /= 3;
        }

        let mut min_counter = 0;
        let mut max_counter = 0;

        let mut pow_3 = 1;

        for i in 0..bit.len() {
            if bit[i] != 0 {
                min_counter += bit[i];
                max_counter += pow_3 * bit[i];
            }

            pow_3 *= 3;
        }

        if min_counter <= k && k <= max_counter && (k - min_counter) % 2 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
