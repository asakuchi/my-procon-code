use std::collections::HashSet;

fn main() {
    let n = 3;
    let k = 3;
    let a = vec![1, 3, 4];
    let x = vec![5, 6, 7];

    solve(n, k, &a, &x);

    let n = 3;
    let k = 3;
    let a = vec![1, 3, 4];
    let x = vec![5, 6, 8];

    solve(n, k, &a, &x);
}

fn solve(n: usize, k: usize, a: &Vec<usize>, x: &Vec<usize>) {
    let mut xor = 0;

    let mut memo = vec![None; 10_001];

    for i in 0..n {
        xor ^= grundy(k, &a, x[i], &mut memo);
    }

    if xor != 0 {
        println!("Alice");
    } else {
        println!("Bob");
    }
}

fn grundy(k: usize, a: &Vec<usize>, x: usize, memo: &mut Vec<Option<usize>>) -> usize {
    if let Some(value) = memo[x] {
        return value;
    }

    if x == 0 {
        return 0;
    }

    let mut set = HashSet::new();

    for i in 0..k {
        if x >= a[i] {
            set.insert(grundy(k, a, x - a[i], memo));
        }
    }

    let mut g = 0;

    while set.contains(&g) {
        g += 1;
    }

    memo[x] = Some(g);

    g
}
