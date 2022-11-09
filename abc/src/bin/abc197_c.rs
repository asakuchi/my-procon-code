use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 1_000_000_000_000;

    for mask in 0..1 << (n - 1) {
        // println!("{:03b}", mask);

        let mut candi = 0;
        let mut chunk = vec![a[0]];

        for i in 0..n - 1 {
            if mask & 1 << i > 0 {
                // ここで分割する
                candi ^= or(&chunk);
                chunk.clear();
            }

            chunk.push(a[i + 1]);
        }

        if chunk.len() > 0 {
            candi ^= or(&chunk);
        }

        result = result.min(candi);
    }

    println!("{}", result);
}

fn or(list: &[usize]) -> usize {
    let mut result = list[0];

    for i in 1..list.len() {
        result |= list[i];
    }

    // println!("OR {:?} : {}", list, result);

    result
}
