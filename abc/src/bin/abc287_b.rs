use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut result = 0;

    for i in 0..n {
        let mut ok = false;

        for j in 0..m {
            if &s[i][3..] == t[j] {
                ok = true;
                break;
            }
        }

        if ok {
            result += 1;
        }
    }

    println!("{}", result);
}
