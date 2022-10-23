use proconio::input;

fn main() {
    input! {
        n: usize,
        goal: (isize, isize),
        a: [isize; n],
    }

    // x について
    let start = a[0];

    let list: Vec<_> = a[2..].iter().step_by(2).map(|&x| x).collect();
    let mut dp = vec![vec![None; 20_050]; n];
    let list_size = list.len();

    let result_x = rec(list_size, goal.0, &list, 0, start, &mut dp);

    // y について
    let start = 0;

    let list: Vec<_> = a[1..].iter().step_by(2).map(|&x| x).collect();
    let mut dp = vec![vec![None; 20_050]; n];
    let list_size = list.len();

    let result_y = rec(list_size, goal.1, &list, 0, start, &mut dp);

    if result_x && result_y {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    goal: isize,
    a: &Vec<isize>,
    index: usize,
    current: isize,
    dp: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if index == n {
        return current == goal;
    }

    // dp
    if let Some(value) = dp[index][(current + 10_000) as usize] {
        return value;
    }

    let diff = a[index];

    for pattern in vec![-diff, diff] {
        let next = current + pattern;

        if rec(n, goal, a, index + 1, next, dp) {
            return true;
        }
    }

    // dp
    dp[index][(current + 10_000) as usize] = Some(false);

    false
}
