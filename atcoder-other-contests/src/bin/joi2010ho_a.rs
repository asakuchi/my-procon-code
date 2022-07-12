use proconio::input;

const MOD: usize = 100_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n - 1],
        a: [isize; m],
    }

    let mut acc = vec![0; n];

    for i in 0..n - 1 {
        acc[i + 1] += acc[i] + s[i];
    }

    // println!("{:?}", acc);

    let mut result = 0;

    let mut current = 0;

    for diff in a.iter() {
        let next = (current as isize + diff) as usize;

        // println!("current:{} next:{}", current + 1, next + 1);

        if next > current {
            result += acc[next] - acc[current];
        } else {
            result += acc[current] - acc[next];
        }

        result %= MOD;

        current = next;
    }

    println!("{}", result);
}
