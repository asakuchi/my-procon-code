use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut start_index = None;

    let mut zero_size = 0;

    // 頭とお尻が繋がっている
    if a[0] == 0 && a[a.len() - 1] == m - 1 {
        for i in 1..n {
            if a[i - 1] == a[i] || a[i - 1] + 1 == a[i] {
                zero_size += a[i];
            } else {
                start_index = Some(i);
                break;
            }
        }
    } else {
        start_index = Some(0);
    }

    if let None = start_index {
        println!("{}", 0);
        return;
    }

    let start_index = start_index.unwrap();

    let mut max_group_size = 0;

    let mut group_size = 0;

    for i in start_index..n {
        group_size += a[i];

        // 連続していない or 終わり
        if i == n - 1 || (a[i] != a[i + 1] && a[i] + 1 != a[i + 1]) {
            if i == n - 1 {
                group_size += zero_size;
            }

            max_group_size = max_group_size.max(group_size);

            group_size = 0;
        }
    }

    let total: usize = a.iter().sum();

    println!("{}", total - max_group_size);
}
