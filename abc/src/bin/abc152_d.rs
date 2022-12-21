use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut count = vec![vec![0; 10]; 10];

    for i in 1..=n {
        let text: Vec<_> = i.to_string().chars().collect();

        let head: usize = text[0].to_string().parse().unwrap();
        let tail: usize = text[text.len() - 1].to_string().parse().unwrap();

        count[head][tail] += 1;
    }

    let mut result = 0;

    for head in 0..=9 {
        for tail in 0..=9 {
            result += count[head][tail] * count[tail][head];
        }
    }

    println!("{}", result);
}
