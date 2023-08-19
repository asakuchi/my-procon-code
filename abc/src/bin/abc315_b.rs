use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    }

    let all: usize = d.iter().sum();

    let target = all / 2 + 1;

    let mut count = 0;

    for i in 0..m {
        if target <= count + d[i] {
            // この月

            for j in 1..=d[i] {
                count += 1;

                if count == target {
                    println!("{} {}", i + 1, j);
                    return;
                }
            }
        }

        count += d[i];
    }
}
