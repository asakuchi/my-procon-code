use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }

    let mut sum = 0;

    for c in n {
        let number: usize = c.to_string().parse().unwrap();
        sum += number;
    }

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
