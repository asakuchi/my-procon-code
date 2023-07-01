use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }

    for i in 0..8 {
        if i != 0 && s[i - 1] > s[i] {
            println!("No");
            return;
        }

        if s[i] < 100 || s[i] > 675 {
            println!("No");
            return;
        }

        if s[i] % 25 != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
