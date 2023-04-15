use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }

    for _ in 0..10 {
        if check(n, &a, &b) {
            println!("Yes");
            return;
        }

        a = rotate(n, &a);
    }

    println!("No");
}

fn rotate(n: usize, a: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[n - 1 - j][i];
        }
    }

    result
}

fn check(n: usize, a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> bool {
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                if b[i][j] != 1 {
                    return false;
                }
            }
        }
    }

    true
}
