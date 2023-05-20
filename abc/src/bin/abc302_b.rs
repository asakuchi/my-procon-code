use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let directions = vec![
        (1_isize, 0_isize),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let snuke = vec!['s', 'n', 'u', 'k', 'e'];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 's' {
                for &direction in directions.iter() {
                    let mut ok = true;

                    for k in 0..snuke.len() {
                        let next_i = i as isize + direction.0 * k as isize;
                        let next_j = j as isize + direction.1 * k as isize;

                        if next_i < 0 || next_j < 0 || next_i >= h as isize || next_j >= w as isize
                        {
                            ok = false;
                            break;
                        }

                        let next_i = next_i as usize;
                        let next_j = next_j as usize;

                        if s[next_i][next_j] != snuke[k] {
                            ok = false;
                            break;
                        }
                    }

                    if ok {
                        for k in 0..snuke.len() {
                            let next_i = i as isize + direction.0 * k as isize;
                            let next_j = j as isize + direction.1 * k as isize;

                            let next_i = next_i as usize;
                            let next_j = next_j as usize;

                            println!("{} {}", next_i + 1, next_j + 1);
                        }

                        return;
                    }
                }
            }
        }
    }
}
