use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n ],
    }

    let mut max = 0;
    let mut max_2 = 0;

    for i in 0..n {
        if a[i] > max {
            max_2 = max;
            max = a[i];
        } else if a[i] > max_2 {
            max_2 = a[i];
        }
    }

    for i in 0..n {
        if a[i] == max {
            println!("{}", max_2);
        } else {
            println!("{}", max);
        }
    }
}
