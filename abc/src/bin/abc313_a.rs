use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let &max = a.iter().max().unwrap();

    let mut count = 0;

    for i in 1..n {
        if a[i] == a[0] {
            count += 1;
        }
    }

    if a[0] == max && count == 0 {
        println!("{}", 0);
    } else {
        println!("{}", max - a[0] + 1);
    }
}
