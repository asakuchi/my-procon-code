use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut colors = 1;

    let mut current_highest = a[0];

    for i in 1..n {
        if a[i] > current_highest {
            colors += 1;
            current_highest = a[i];
        }
    }

    if colors >= k {
        println!("0");
        return;
    }

    let mut result: usize = 1_000_000_000_000;

    for mask in 0..1 << n {
        if mask & 1 > 0 {
            continue;
        }

        let mut colors = 1;
        let mut current_highest = a[0];

        let mut cost = 0;

        for i in 1..n {
            if a[i] > current_highest {
                colors += 1;
                current_highest = a[i];
                continue;
            }

            if mask & 1 << i > 0 {
                cost += (current_highest + 1) - a[i];
                colors += 1;
                current_highest += 1;
            }
        }

        if colors < k {
            continue;
        }

        result = result.min(cost);
    }

    println!("{}", result);
}
