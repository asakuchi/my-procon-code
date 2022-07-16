use proconio::input;

fn main() {
    input! {
        mut p: usize,
    }

    let mut list = Vec::new();

    let mut current = 1;

    for i in 1..=100 {
        current *= i;

        if current > p {
            break;
        }

        list.push(current);
    }

    list.reverse();

    let mut result = 0;

    for &coin in list.iter() {
        while coin <= p {
            p -= coin;
            result += 1;
        }
    }

    println!("{}", result);
}
