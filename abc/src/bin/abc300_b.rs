use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }

    let mut list = a.clone();

    for _ in 0..=h {
        let mut list_2 = list.clone();

        for _ in 0..=w {
            if list_2 == b {
                println!("Yes");
                return;
            }

            list_2 = yoko(h, w, &list_2);
        }

        list = tate(h, w, &list);
    }

    println!("No");
}

fn tate(h: usize, w: usize, a: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec![' '; w]; h];

    for i in 0..h - 1 {
        for j in 0..w {
            result[i][j] = a[i + 1][j];
        }
    }

    for j in 0..w {
        result[h - 1][j] = a[0][j];
    }

    result
}

fn yoko(h: usize, w: usize, a: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec![' '; w]; h];

    for i in 0..h {
        for j in 0..w - 1 {
            result[i][j] = a[i][j + 1];
        }
    }

    for i in 0..h {
        result[i][w - 1] = a[i][0];
    }

    result
}
