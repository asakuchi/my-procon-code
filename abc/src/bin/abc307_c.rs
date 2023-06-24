use proconio::{input, marker::Chars};

const SIZE: usize = 20;

fn main() {
    input! {
        h_a: usize,
        w_a: usize,
        a: [Chars; h_a],
        h_b: usize,
        w_b: usize,
        b: [Chars; h_b],
        h_x: usize,
        w_x: usize,
        x: [Chars; h_x],
    }

    'search: for i_x in 0..h_x {
        for j_x in 0..w_x {
            let mut filled = vec![vec!['.'; SIZE]; SIZE];

            // (i_x, j_x) からシートAを貼ってみる
            for i_a in 0..h_a {
                for j_a in 0..w_a {
                    if a[i_a][j_a] == '#' {
                        filled[i_x + i_a][j_x + j_a] = '#';

                        // 外はNG
                        if (i_x + i_a >= h_x) || (j_x + j_a >= w_x) {
                            continue 'search;
                        }
                    }
                }
            }

            // 更に 別の (i_x, j_x) から シートB を貼ってみる
            'search_b: for i_x_2 in 0..h_x {
                for j_x_2 in 0..w_x {
                    let mut filled_2 = filled.clone();

                    for i_b in 0..h_b {
                        for j_b in 0..w_b {
                            if b[i_b][j_b] == '#' {
                                filled_2[i_x_2 + i_b][j_x_2 + j_b] = '#';

                                // 外はNG
                                if (i_x_2 + i_b >= h_x) || (j_x_2 + j_b >= w_x) {
                                    continue 'search_b;
                                }
                            }
                        }
                    }

                    // シートXの # を全て含むか
                    let mut ok = true;

                    for i in 0..h_x {
                        for j in 0..w_x {
                            if x[i][j] != filled_2[i][j] {
                                ok = false;
                                break;
                            }
                        }
                    }

                    if ok {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
