use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut list = Vec::new();

    for _ in 0..n {
        input! {
            c: usize,
            a: [usize; c],
        }

        list.push(a);
    }

    input! {
        x: usize,
    }

    let mut me_list = vec![Vec::new(); 38];

    for i in 0..n {
        if list[i].contains(&x) {
            me_list[list[i].len()].push(i);
        }
    }

    for i in 0..38 {
        if me_list[i].len() > 0 {
            println!("{}", me_list[i].len());

            let text = me_list[i].iter().map(|x| x + 1).format(" ");
            println!("{}", text);

            return;
        }
    }

    println!("0");
}
