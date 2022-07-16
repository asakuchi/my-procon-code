use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut result = 1_000_000_000;

    'target_loop: for target in 0..=9 {
        let mut stopped = vec![false; n];

        let mut count = 0;

        for time in 0..1_000_000_000 {
            for i in 0..n {
                if !stopped[i] && s[i][time % 10].to_string().parse::<usize>().unwrap() == target {
                    stopped[i] = true;
                    count += 1;
                    break;
                }
            }

            if count == n {
                result = result.min(time);

                continue 'target_loop;
            }
        }
    }

    println!("{}", result);
}
