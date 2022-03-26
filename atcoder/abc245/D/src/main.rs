use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n + 1],
        mut c: [isize; n + m + 1],
    }

    let mut b = Vec::new();

    a.reverse();
    c.reverse();

    let mut i = 0;

    loop {
        let c_value = c[i];
        let quo = c_value / a[0];
        b.push(quo);

        // eprintln!("c:{} a[0]:{} b:{:?}", c_value, a[0], b);

        c[i] -= quo;

        for j in 1..a.len() {
            c[i + j] -= a[j] * quo;
        }

        // eprintln!("after:c:{:?}", c);

        if b.len() == m + 1 {
            break;
        }

        i += 1;
    }

    b.reverse();

    let result = b
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
}
