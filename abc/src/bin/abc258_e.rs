use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: usize,
        w: [usize; n],
    }

    let mut all_weight = 0;

    for i in 0..n {
        all_weight += w[i];
    }

    let mut not_loop = Vec::new();
    let mut loop_package = Vec::new();

    let mut package_size = std::collections::HashMap::new();

    let mut map = std::collections::HashMap::new();

    let mut i = 0;

    loop {
        let mut weight = 0;

        let count = map.entry(i).or_insert(0);
        *count += 1;

        if *count == 1 {
            not_loop.push(i);
        }

        if *count == 2 {
            loop_package.push(i);
        }

        if *count == 3 {
            break;
        }

        let start_i = i;

        if let Some(&(_count, next_i)) = package_size.get(&start_i) {
            i = next_i;
        } else {
            let mut count = 0;

            let mut ikiti = x;

            if all_weight < x {
                count += (x / all_weight) * n;
                ikiti %= all_weight;
            }

            while weight < ikiti {
                weight += w[i];
                count += 1;

                i += 1;
                if i >= n {
                    i = 0;
                }
            }

            package_size.insert(start_i, (count, i));
        }
    }

    for _ in 0..q {
        input! { mut k : Usize1};

        if k <= not_loop.len() {
            let index = not_loop[k];

            let (size, _) = package_size.get(&index).unwrap();

            println!("{}", size);
        } else {
            k -= not_loop.len();

            k %= loop_package.len();

            let index = loop_package[k];

            let (size, _) = package_size.get(&index).unwrap();

            println!("{}", size);
        }
    }
}
