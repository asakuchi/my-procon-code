use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut sorted = a.clone();
    sorted.sort();

    let mut plane_list = vec![Vec::new(); k];
    let mut ordered_list = vec![Vec::new(); k];

    for i in 0..n {
        plane_list[i % k].push(a[i]);
        ordered_list[i % k].push(sorted[i]);
    }

    for i in 0..k {
        plane_list[i].sort();
        ordered_list[i].sort();

        if plane_list[i] != ordered_list[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
