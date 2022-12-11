use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let result = rec(n, &a, 30);

    println!("{}", result);
}

fn rec(n: usize, list: &Vec<usize>, bit: isize) -> usize {
    if bit == -1 {
        return 0;
    }

    let mut list_0 = Vec::new();
    let mut list_1 = Vec::new();

    let mask = 1 << bit;

    for &value in list {
        if mask & value == 0 {
            list_0.push(value);
        } else {
            list_1.push(value);
        }
    }

    if list_0.len() == 0 || list_1.len() == 0 {
        return rec(n, list, bit - 1);
    }

    let score_1 = rec(n, &list_0, bit - 1);
    let score_2 = rec(n, &list_1, bit - 1);

    score_1.min(score_2) | 1 << bit
}
