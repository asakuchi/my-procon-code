// use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

// #[fastout]
fn main() {
    input! {
        mut n: Chars,
        k: usize,
    }

    n.reverse();

    let mut base_8: Vec<usize> = n.iter().map(|c| c.to_string().parse().unwrap()).collect();

    for _ in 0..k {
        let mut number = 0;

        // eprintln!("base8:{:?}", base_8);

        for i in 0..base_8.len() {
            number += base_8[i] * (8 as usize).pow(i as u32);
        }

        let mut base_9 = Vec::new();

        while number != 0 {
            base_9.push(number % 9);
            number /= 9;
        }

        // eprintln!("base9:{:?}", base_9);

        base_8 = base_9.iter().map(|&x| if x == 8 { 5 } else { x }).collect();
    }

    base_8.reverse();

    if base_8.len() == 0 {
        print!("0");
    } else {
        for num in base_8.iter() {
            print!("{}", num);
        }
    }

    println!();
}
