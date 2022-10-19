use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut result = 0;

    'search: for i in a..=b {
        let text: Vec<_> = i.to_string().chars().collect();

        for j in 0..text.len() {
            if text[j] != text[text.len() - 1 - j] {
                continue 'search;
            }
        }

        result += 1;
    }

    println!("{}", result);
}
