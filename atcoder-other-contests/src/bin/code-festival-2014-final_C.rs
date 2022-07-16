// use proconio::fastout;
use proconio::input;

// #[fastout]
fn main() {
    input! {
        mut a: usize,
    }

    let mut result = -1;

    'k_loop: for k in 10..=10000 {
        let mut candi = Vec::new();

        let mut number = a;

        while number != 0 {
            candi.push(number % k);
            number /= k;
        }

        candi.reverse();

        // check
        let k_str = k.to_string();

        if candi.len() != k_str.len() {
            continue 'k_loop; // NG
        }

        for (i, c) in k_str.chars().enumerate() {
            if candi[i] != c.to_string().parse::<usize>().unwrap() {
                continue 'k_loop; // NG
            }
        }

        let result_str: String = candi.iter().map(|x| x.to_string()).collect();
        result = result_str.parse().unwrap();
        break;
    }

    println!("{}", result);
}
