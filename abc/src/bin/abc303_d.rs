use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }

    let list = run_length_encoding(&s);

    let mut dp = vec![vec![1_000_000_000_000_000; 2]; list.len() + 1];

    // Caps OFF
    dp[0][0] = 0;

    for i in 0..list.len() {
        let (c, size) = list[i];

        if c == 'A' {
            // Caps OFFのまま
            let off = dp[i][0] + y * size;
            dp[i + 1][0] = dp[i + 1][0].min(off);

            // Caps ON -> OFF
            let from_on = dp[i][1] + z + y * size;
            dp[i + 1][0] = dp[i + 1][0].min(from_on);

            // Caps ON のまま
            let on = dp[i][1] + x * size;
            dp[i + 1][1] = dp[i + 1][1].min(on);

            // Caps OFF -> ON
            let to_on = dp[i][0] + z + x * size;
            dp[i + 1][1] = dp[i + 1][1].min(to_on);
        } else {
            // Caps OFFのまま
            let off = dp[i][0] + x * size;
            dp[i + 1][0] = dp[i + 1][0].min(off);

            // Caps ON -> OFF
            let from_on = dp[i][1] + z + x * size;
            dp[i + 1][0] = dp[i + 1][0].min(from_on);

            // Caps ON のまま
            let on = dp[i][1] + y * size;
            dp[i + 1][1] = dp[i + 1][1].min(on);

            // Caps OFF -> ON
            let to_on = dp[i][0] + z + y * size;
            dp[i + 1][1] = dp[i + 1][1].min(to_on);
        }
    }

    println!("{}", dp[list.len()][0].min(dp[list.len()][1]));
}

///
/// ランレングス圧縮
///
pub fn run_length_encoding<T>(list: &Vec<T>) -> Vec<(T, usize)>
where
    T: PartialEq + Copy,
{
    let mut result = Vec::new();

    for &value in list.iter() {
        if let Some((next, size)) = result.pop() {
            if next == value {
                result.push((next, size + 1));
            } else {
                result.push((next, size));
                result.push((value, 1));
            }
        } else {
            result.push((value, 1));
        }
    }

    result
}
