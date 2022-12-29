use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut tiles_size = vec![0usize; 2 * h * w + 2];
    let mut tile_index = 1;

    let mut tile_yoko = vec![vec![0; w]; h];
    let mut tile_tate = vec![vec![0; w]; h];

    // 横方向
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                if j > 0 {
                    if s[i][j - 1] == '#' {
                        tile_index += 1;
                    }
                }

                tile_yoko[i][j] = tile_index;
                tiles_size[tile_index] += 1;
            }
        }

        tile_index += 1;
    }

    // 縦方向
    for j in 0..w {
        for i in 0..h {
            if s[i][j] == '.' {
                if i > 0 {
                    if s[i - 1][j] == '#' {
                        tile_index += 1;
                    }
                }

                tile_tate[i][j] = tile_index;
                tiles_size[tile_index] += 1;
            }
        }

        tile_index += 1;
    }

    // println!("{:?}", tile_yoko);
    // println!("{:?}", tile_tate);
    // println!("{:?}", tiles_size);

    let mut result = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            let index_yoko = tile_yoko[i][j];
            let yoko_size = tiles_size[index_yoko];

            let index_tate = tile_tate[i][j];
            let tate_size = tiles_size[index_tate];

            // println!(
            //     "y_i {} y_s {} t_i {} t_s {}",
            //     index_yoko, yoko_size, index_tate, tate_size
            // );

            result = result.max(yoko_size + tate_size - 1);
        }
    }

    println!("{}", result);
}
