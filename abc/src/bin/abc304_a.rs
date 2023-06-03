use proconio::input;

fn main() {
    input! {
        n: usize,
        s_a: [(String, usize); n],
    }

    let mut min = s_a[0].1;
    let mut min_index = 0;

    for i in 0..n {
        if min > s_a[i].1 {
            min = s_a[i].1;
            min_index = i;
        }
    }

    let mut index = min_index;

    for _ in 0..n {
        println!("{}", s_a[index % n].0);
        index += 1;
    }
}
