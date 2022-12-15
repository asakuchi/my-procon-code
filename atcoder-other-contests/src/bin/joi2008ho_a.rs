use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }

    let mut stone = Vec::new();

    for i in 0..n {
        if i % 2 == 0 {
            // そのまま入れる
            if let Some((color, size)) = stone.pop() {
                if color == c[i] {
                    stone.push((color, size + 1));
                } else {
                    stone.push((color, size));
                    stone.push((c[i], 1));
                }
            } else {
                stone.push((c[i], 1));
            }
        } else {
            // 右端を考慮

            if let Some((color, size)) = stone.pop() {
                if color == c[i] {
                    stone.push((color, size + 1));
                } else {
                    // 色を変える

                    // もう一度popすると同じ色のはず
                    if let Some((_, size_2)) = stone.pop() {
                        stone.push((c[i], size + size_2 + 1));
                    } else {
                        stone.push((c[i], size + 1));
                    }
                }
            } else {
                panic!("cant pop stone");
            }
        }
    }

    let result: usize = stone
        .iter()
        .filter(|&(color, _size)| *color == 0)
        .fold(0, |acc, &(_color, size)| acc + size);

    println!("{}", result);
}
