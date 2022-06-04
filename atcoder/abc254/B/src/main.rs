use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut list = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..=i {
            if j == 0 || j == i {
                list[i][j] = 1;
            } else {
                list[i][j] = list[i - 1][j - 1] + list[i - 1][j];
            }
        }

        let line = &list[i][0..=i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", line);
    }
}
