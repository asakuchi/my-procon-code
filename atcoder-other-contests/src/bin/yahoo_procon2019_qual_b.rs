use proconio::{input, marker::Usize1};

fn main() {
    let mut list = vec![Vec::new(); 4];

    for _ in 0..3 {
        input! {
            a: Usize1,
            b: Usize1,
        }

        list[a].push(b);
        list[b].push(a);
    }

    for i in 0..4 {
        if list[i].len() != 1 && list[i].len() != 2 {
            println!("NO");
            return;
        }
    }

    println!("YES");
}
