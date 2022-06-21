use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        m: usize,
        mut shop: [usize; n - 1],
        mut k: [usize; m],
    }

    shop.push(0);
    shop.push(d);
    shop.sort();

    k.sort();

    let mut result = 0;

    let mut i = 0;

    for j in 0..m {
        while k[j] > shop[i + 1] {
            i += 1;
        }

        if k[j] - shop[i] < shop[i + 1] - k[j] {
            result += k[j] - shop[i];
        } else {
            result += shop[i + 1] - k[j];
        }
    }

    println!("{}", result);
}
