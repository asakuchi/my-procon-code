use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s_c: [(usize, usize); m],
    }

    let range = match n {
        1 => (0..=9),
        2 => (10..=99),
        _ => (100..=999),
    };

    'seach_num: for candi in range {
        for &(s, c) in &s_c {
            if candi / 10usize.pow((n - s) as u32) % 10 != c {
                continue 'seach_num;
            }
        }

        println!("{}", candi);
        return;
    }

    println!("-1");
}
