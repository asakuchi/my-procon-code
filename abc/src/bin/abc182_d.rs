use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut s = vec![0; n + 1];
    // i以前での最大
    let mut ruiseki_max = vec![0; n + 1];

    for i in 0..n {
        s[i + 1] = s[i] + a[i];

        ruiseki_max[i + 1] = ruiseki_max[i].max(s[i + 1]);
    }

    // println!("{:?}", s);
    // println!("{:?}", ruiseki_max);

    let mut position = 0;
    let mut position_max = 0;

    // n+1 ではなく n まで
    for i in 0..n + 1 {
        if position + ruiseki_max[i] > position_max {
            position_max = position + ruiseki_max[i];
        }

        position += s[i];
    }

    println!("{}", position_max);
}
