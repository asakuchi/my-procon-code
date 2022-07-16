use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut amount = 0;

    let mut map = std::collections::HashMap::new();

    let mut list = Vec::new();

    let mut amount_per_loop = 0;

    for _ in 0..k {
        let index = amount % n;

        let times = map.entry(index).or_insert(0);

        if *times == 1 {
            list.push(index);
            amount_per_loop += a[index];
        } else if *times == 2 {
            break;
        }

        *times += 1;

        amount += a[index];
        k -= 1;

        if k == 0 {
            println!("{}", amount);
            return;
        }
    }

    amount += (k / list.len()) * amount_per_loop;
    k = k % list.len();

    for _ in 0..k {
        let index = amount % n;
        amount += a[index];
    }

    println!("{}", amount);
}
