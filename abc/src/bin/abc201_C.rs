use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut included = Vec::new();
    let mut not = Vec::new();

    for i in 0..=9 {
        match s[i] {
            'o' => included.push(i),
            'x' => not.push(i),
            _ => {}
        }
    }

    let mut result = 0;

    'num_loop: for num in 0..=9999 {
        let text = format!("{:04}", num);

        for digit in included.iter() {
            if !text.contains(&digit.to_string()) {
                continue 'num_loop;
            }
        }

        for digit in not.iter() {
            if text.contains(&digit.to_string()) {
                continue 'num_loop;
            }
        }

        result += 1;
    }

    println!("{}", result);
}
