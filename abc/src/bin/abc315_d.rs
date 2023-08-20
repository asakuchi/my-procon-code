//
// 解説AC
//

use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut rows = vec![HashMap::new(); h];
    let mut columns = vec![HashMap::new(); w];

    for i in 0..h {
        for j in 0..w {
            *rows[i].entry(a[i][j]).or_insert(0) += 1;
            *columns[j].entry(a[i][j]).or_insert(0) += 1;
        }
    }

    loop {
        let mut delete_rows = HashSet::new();

        for i in 0..h {
            if rows[i].len() == 1 {
                let (_, &value) = rows[i].iter().next().unwrap();

                if value > 1 {
                    delete_rows.insert(i);
                }
            }
        }

        let mut delete_columns = HashSet::new();

        for j in 0..w {
            if columns[j].len() == 1 {
                let (_, &value) = columns[j].iter().next().unwrap();

                if value > 1 {
                    delete_columns.insert(j);
                }
            }
        }

        // ------------------------------

        for &i in delete_rows.iter() {
            let (&c, _) = rows[i].iter().next().unwrap();

            for j in 0..w {
                if let Some(value) = columns[j].get_mut(&c) {
                    if *value > 1 {
                        *value -= 1;
                    } else {
                        columns[j].remove_entry(&c);
                    }
                }
            }

            rows[i].remove_entry(&c);
        }

        for &j in delete_columns.iter() {
            // 上の行単位の削除で消えて無くなっている場合がある
            if let Some((&c, _)) = columns[j].iter().next() {
                for i in 0..h {
                    if let Some(value) = rows[i].get_mut(&c) {
                        if *value > 1 {
                            *value -= 1;
                        } else {
                            rows[i].remove_entry(&c);
                        }
                    }
                }

                columns[j].remove_entry(&c);
            }
        }

        // ------------------------------

        if delete_rows.len() == 0 && delete_columns.len() == 0 {
            break;
        }
    }

    let mut result = 0_usize;

    for i in 0..h {
        for c in 'a'..='z' {
            if let Some(value) = rows[i].get(&c) {
                result += value;
            }
        }
    }

    println!("{}", result);
}
