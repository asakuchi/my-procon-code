use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut nums: [usize; 3],
    }

    nums.sort();

    if nums[2] > nums[0] + nums[1] {
        println!("-1");
        return;
    }

    let result = rec(nums[0], nums[1], nums[2]);

    println!("{}", result);
}

fn rec(a: usize, b: usize, c: usize) -> usize {
    let mut list = [a, b, c];
    list.sort();
    let [a, b, c] = list;

    if a == b && b == c {
        return a;
    }

    if a == b {
        return rec(a - 1, b, c - 1) + 1;
    }

    if b - a > c - b {
        let diff = b - a;
        return rec(a, b - diff, c - diff) + diff;
    } else {
        let diff = c - b;
        return rec(a, b - diff, c - diff) + diff;
    }
}
