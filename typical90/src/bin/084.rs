use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut right = 0;

    let mut count_o = 0;
    let mut count_x = 0;

    let mut result = 0;

    if s[0] == 'o' {
        count_o += 1;
    } else {
        count_x += 1;
    }

    for left in 0..n {
        while count_o == 0 || count_x == 0 {
            right += 1;

            if right == n {
                break;
            }

            if s[right] == 'o' {
                count_o += 1;
            } else {
                count_x += 1;
            }
        }

        if right == n {
            break;
        }

        result += n - right;

        if s[left] == 'o' {
            count_o -= 1;
        } else {
            count_x -= 1;
        }
    }

    println!("{}", result);
}
