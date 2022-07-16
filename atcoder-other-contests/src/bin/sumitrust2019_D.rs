use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut result = 0;

    for i in 0..=9 {
        let i_index;

        if let Some(position) = s[..n - 2]
            .iter()
            .position(|&c| c.to_string() == i.to_string())
        {
            i_index = position;
        } else {
            continue;
        }

        for j in 0..=9 {
            let j_index;

            if let Some(position) = s[i_index + 1..n - 1]
                .iter()
                .position(|&c| c.to_string() == j.to_string())
            {
                j_index = position;
            } else {
                continue;
            }

            for k in 0..=9 {
                // j_index は s[i_index + 1..n - 1] に対してのindexなので注意
                if let Some(_) = s[i_index + 1 + j_index + 1..]
                    .iter()
                    .position(|&c| c.to_string() == k.to_string())
                {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}
