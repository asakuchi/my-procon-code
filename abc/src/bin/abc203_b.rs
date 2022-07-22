use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut answer = 0;

    for i in 1..=n {
        for j in 1..=k {
            let text = format!("{}0{}", i, j);

            let number: usize = text.parse().unwrap();
            answer += number;
        }
    }

    println!("{}", answer);
}
