use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    }

    let mut result = 0;

    let mut renzoku = 0;

    for i in 0..d {
        let mut ok = true;

        for j in 0..n {
            if s[j][i] == 'x' {
                ok = false;
                break;
            }
        }

        if ok {
            renzoku += 1;
        } else {
            result = result.max(renzoku);

            renzoku = 0;
        }
    }

    result = result.max(renzoku);

    println!("{}", result);
}
