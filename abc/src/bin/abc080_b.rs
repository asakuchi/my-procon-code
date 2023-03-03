use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let sum: usize = n
        .to_string()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .sum();

    if n % sum == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
