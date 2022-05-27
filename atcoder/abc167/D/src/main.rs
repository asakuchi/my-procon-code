use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [Usize1; n],
    }

    let mut list = Vec::new();
    let mut map = std::collections::HashMap::new();

    let mut town = 0;

    loop {
        let count = map.entry(town).or_insert(0);

        if *count == 1 {
            list.push(town);
        } else if *count == 2 {
            break;
        }

        *count += 1;
        town = a[town];

        k -= 1;

        if k == 0 {
            println!("{}", town + 1);
            return;
        }
    }

    k = k % list.len();

    println!("{} ", list[k] + 1);
}
