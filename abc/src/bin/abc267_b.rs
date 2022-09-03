use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    if s[0] == '1' {
        println!("No");
        return;
    }

    let rows = vec![
        vec![7],
        vec![4],
        vec![2, 8],
        vec![1, 5],
        vec![3, 9],
        vec![6],
        vec![10],
    ];

    let mut flags = vec![false; 7];

    for i in 0..7 {
        let mut ari = false;
        let row = rows[i].clone();

        for &pin in row.iter() {
            if s[pin - 1] == '1' {
                ari = true;
            }
        }

        flags[i] = ari;
    }

    for i in 0..7 {
        let mut has_none = false;

        for j in i + 2..7 {
            if !flags[j] {
                has_none = true;
            }

            if flags[i] && flags[j] && has_none {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
