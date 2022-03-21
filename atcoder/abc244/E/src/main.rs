use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const MODULO: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        mut uv: [(Usize1, Usize1); m],
    }

    let mut list = vec![vec![]; n];

    for &(u, v) in uv.iter() {
        list[u].push(v);
        list[v].push(u);
    }

    let mut memo = vec![vec![vec![-1; 2]; k + 1]; n];

    let result = rec(&list, n, m, t, x, s, k, true, &mut memo);

    println!("{}", result % MODULO);
}

fn rec(
    list: &Vec<Vec<usize>>,
    n: usize,
    m: usize,
    t: usize,
    x: usize,
    current: usize,
    rest: usize,
    is_x_even: bool,
    memo: &mut Vec<Vec<Vec<isize>>>,
) -> usize {
    if rest == 0 {
        if current == t && is_x_even {
            return 1;
        } else {
            return 0;
        }
    }

    if memo[current][rest][is_x_even as usize] != -1 {
        return memo[current][rest][is_x_even as usize] as usize;
    }

    let mut result = 0;

    let is_x_even_next = if current == x { !is_x_even } else { is_x_even };

    for &next in list[current].iter() {
        result += rec(list, n, m, t, x, next, rest - 1, is_x_even_next, memo);
        result %= MODULO;
    }

    result %= MODULO;

    memo[current][rest][is_x_even as usize] = result as isize;

    result
}
