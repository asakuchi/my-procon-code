use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h],
    }

    for i in 0..h {
        let mut result = Vec::new();

        for j in 0..w {
            if a[i][j] == 0 {
                result.push('.');
            } else {
                let c = 'A' as u8 + a[i][j] - 1;

                result.push(c as char);
            }
        }

        let text: String = result.into_iter().collect();
        println!("{}", text);
    }
}
