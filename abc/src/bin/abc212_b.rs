use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    }

    let nums: Vec<usize> = x.iter().map(|c| c.to_string().parse().unwrap()).collect();

    let mut all_same = true;
    let mut stair = true;

    for i in 0..3 {
        if nums[i] != nums[i + 1] {
            all_same = false;
        }

        if (nums[i] <= 8 && nums[i] + 1 != nums[i + 1]) || (nums[i] == 9 && nums[i + 1] != 0) {
            stair = false;
        }
    }

    if !all_same && !stair {
        println!("Strong");
    } else {
        println!("Weak");
    }
}
