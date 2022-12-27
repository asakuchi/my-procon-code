use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: usize,
        a_b: [(usize, usize); n],
    }

    // 振るのは最大値のみで良い
    let slash = a_b.iter().map(|&(a, _b)| a).max().unwrap();

    // 投げるのは振るよりダメージがだせるものだけで良い
    let mut throw_list: Vec<_> = a_b
        .iter()
        .map(|&(_a, b)| b)
        .filter(|&b| b > slash)
        .collect();

    throw_list.sort();

    let mut result = 0usize;

    while let Some(katana) = throw_list.pop() {
        result += 1;

        if h > katana {
            h -= katana;
        } else {
            println!("{}", result);
            return;
        }
    }

    if h % slash != 0 {
        result += 1;
    }

    result += h / slash;

    println!("{}", result);
}
