use proconio::input;

fn main() {
    input! {
        n: usize,
        pe: [[(usize, usize)]; n],
    }

    let mut all_lcd = std::collections::HashMap::new();

    for i in 0..n {
        for j in 0..pe[i].len() {
            let (p, e) = pe[i][j];

            let kata = all_lcd.entry(p).or_insert((0, 0));

            if kata.0 < e {
                *kata = (e, 1);
            } else if kata.0 == e {
                kata.1 += 1;
            }
        }
    }

    let mut result = 0;

    for i in 0..n {
        let mut ok = false;

        for j in 0..pe[i].len() {
            let (p, e) = pe[i][j];

            let kata = all_lcd.entry(p).or_insert((0, 0));

            if kata.0 == e && kata.1 == 1 {
                ok = true;
            }
        }

        if ok {
            result += 1;
        }
    }

    if result < n {
        result += 1;
    }

    println!("{}", result);
}
