use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        a_b_c_d: [(Usize1,Usize1,usize,usize); q],
    }

    let result = rec(n, m, q, &a_b_c_d, 0, &mut Vec::new());

    println!("{}", result);
}

fn rec(
    n: usize,
    m: usize,
    q: usize,
    a_b_c_d: &Vec<(usize, usize, usize, usize)>,
    index: usize,
    list: &mut Vec<usize>,
) -> usize {
    // println!("index {} list {:?}", index, list);

    if index == n {
        let mut score = 0;

        for &(a, b, c, d) in a_b_c_d {
            if list[b] - list[a] == c {
                score += d;
            }
        }

        return score;
    }

    let mut result = 0;

    let last_num = if index == 0 { 1 } else { list[index - 1] };

    for a_k in last_num..=m {
        list.push(a_k);

        let score = rec(n, m, q, a_b_c_d, index + 1, list);
        result = result.max(score);

        list.pop();
    }

    result
}
