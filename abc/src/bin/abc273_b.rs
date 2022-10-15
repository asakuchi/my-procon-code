use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
        k: usize,
    }

    let mut numbers: Vec<_> = x
        .iter()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect();

    numbers.reverse();

    for i in 0..k {
        if i >= numbers.len() {
            break;
        }
        let digit = numbers[i];
        if digit >= 5 {
            let mut target = i + 1;

            loop {
                if target == numbers.len() {
                    numbers.push(1);
                    break;
                }

                let upper_digit = numbers[target];

                if upper_digit != 9 {
                    numbers[target] += 1;
                    break;
                }

                numbers[target] = 0;
                target += 1;
            }
        }
        numbers[i] = 0;
    }

    numbers.reverse();

    let result = numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    let result_num: usize = result.parse().unwrap();

    println!("{}", result_num);
}
