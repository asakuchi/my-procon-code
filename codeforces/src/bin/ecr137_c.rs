const INF: isize = 1_000_000_000_000_000;

fn main() {
    let t = input_usize();

    for _ in 0..t {
        let n = input_usize();

        let lid = input_char_vec();

        let amount = input_vec();

        let mut dp = vec![vec![None; 2]; n];

        let magazines = rec(n, &lid, &amount, 0, false, &mut dp);

        println!("{}", magazines);
    }
}

fn input_usize() -> usize {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    n
}

fn input_char_vec() -> Vec<bool> {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let x: Vec<_> = buf.chars().map(|c| c == '1').collect();

    x
}

fn input_vec() -> Vec<isize> {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let line: Vec<_> = iter.map(|x| x.parse().unwrap()).collect();

    line
}

fn rec(
    n: usize,
    lid: &Vec<bool>,
    amount: &Vec<isize>,
    index: usize,
    prev_move: bool,
    dp: &mut Vec<Vec<Option<isize>>>,
) -> isize {
    if index == n {
        return 0;
    }

    if let Some(value) = dp[index][prev_move as usize] {
        return value;
    }

    // 移す
    // 蓋がない、かつ次に蓋がある
    let score_1 = if (!lid[index] || prev_move) && (index < n - 1 && lid[index + 1]) {
        rec(n, lid, amount, index + 1, true, dp) + amount[index]
    } else {
        -INF
    };

    // println!("index {} prev {} 移すscore_1 {}", index, prev_move, score_1);

    // 移さない
    let score_2 = rec(n, lid, amount, index + 1, false, dp)
        + if lid[index] && !prev_move {
            amount[index]
        } else {
            0
        };

    let score = score_1.max(score_2);

    // println!(
    //     "index {} prev {} 移さないscore_2 {}",
    //     index, prev_move, score_2
    // );

    // println!("index {} prev {} score {}", index, prev_move, score);

    dp[index][prev_move as usize] = Some(score);

    score
}
