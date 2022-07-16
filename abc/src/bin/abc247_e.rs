use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
    }

    // 番兵
    a.push(1_000_000_000);

    let mut sub_list = Vec::new();

    let mut result = 0;

    for &value in &a {
        if y <= value && value <= x {
            sub_list.push(value);
        } else {
            result += count_pair(&sub_list, sub_list.len(), x, y);
            sub_list.clear();
        }
    }

    println!("{}", result);
}

fn count_pair(list: &Vec<usize>, n: usize, x: usize, y: usize) -> usize {
    let mut l = 0;

    let mut x_count = 0;
    let mut y_count = 0;

    let mut result = 0;

    for r in 0..n {
        if list[r] == x {
            x_count += 1;
        }

        if list[r] == y {
            y_count += 1;
        }

        while x_count > 0 && y_count > 0 && r >= l {
            result += n - r;

            if list[l] == x {
                x_count -= 1;
            }
            if list[l] == y {
                y_count -= 1;
            }

            l += 1;
        }
    }

    result
}
