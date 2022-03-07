use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::with_capacity(n);
    let mut t = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            time: usize,
            line: [Usize1],
        }

        t.push(time);
        a.push(line);
    }

    let mut mastered = vec![false; n];

    let total = rec(&a, &t, &mut mastered, n - 1);

    println!("{}", total);
}

fn rec(a: &Vec<Vec<usize>>, t: &Vec<usize>, mastered: &mut Vec<bool>, current: usize) -> usize {
    let mut total = 0;

    total += t[current];

    for &required in a[current].iter() {
        if mastered[required] {
            continue;
        }

        total += rec(a, t, mastered, required);
    }

    mastered[current] = true;

    total
}
