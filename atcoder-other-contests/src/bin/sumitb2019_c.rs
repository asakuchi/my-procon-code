use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut list = vec![false; 100001];

    for price in 100..=105 {
        list[price] = true;
    }

    for price in 106..=100000 {
        for diff in 100..=105 {
            if list[price - diff] {
                list[price] = true;
            }
        }
    }

    if list[x] {
        println!("1");
    } else {
        println!("0");
    }
}
