use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut count = 0;

    for text in s {
        if text == "For" {
            count += 1;
        }
    }

    if count > n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
