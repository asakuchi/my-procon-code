use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    'outer: for i in 1..=n {
        let text = i.to_string();

        for c in text.chars() {
            if c == '7' {
                continue 'outer;
            }
        }

        let text = format!("{:o}", i);

        for c in text.chars() {
            if c == '7' {
                continue 'outer;
            }
        }

        result += 1;
    }

    println!("{}", result);
}
