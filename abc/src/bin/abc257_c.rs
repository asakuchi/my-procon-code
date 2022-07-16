use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    }

    let mut adults = Vec::new();
    let mut children = Vec::new();

    for i in 0..n {
        if s[i] == '1' {
            adults.push(w[i]);
        } else {
            children.push(w[i]);
        }
    }

    let adult_count = adults.len();

    let mut list = std::collections::BTreeMap::new();

    for i in 0..n {
        let count = list.entry(w[i]).or_insert((0, 0));
        if s[i] == '1' {
            (*count).1 += 1;
        } else {
            (*count).0 += 1;
        }
    }

    let mut c_count = 0;
    let mut a_count = adult_count;

    let mut max_num = c_count + a_count;

    for (&_w, &count) in list.iter() {
        a_count -= count.1;
        c_count += count.0;

        let score = c_count + a_count;
        max_num = max_num.max(score);
    }

    println!("{}", max_num);
}
