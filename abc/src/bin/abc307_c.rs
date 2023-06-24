use std::collections::HashSet;

use proconio::{input, marker::Chars};

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

    let mut orig_set_a = HashSet::new();

    for i in 0..h_a {
        for j in 0..w_a {
            if a[i][j] == '#' {
                orig_set_a.insert((i as isize, j as isize));
            }
        }
    }

    let mut orig_set_b = HashSet::new();

    for i in 0..h_b {
        for j in 0..w_b {
            if b[i][j] == '#' {
                orig_set_b.insert((i as isize, j as isize));
            }
        }
    }

    let mut set_x = HashSet::new();

    for i in 0..h_x {
        for j in 0..w_x {
            if x[i][j] == '#' {
                set_x.insert((i as isize, j as isize));
            }
        }
    }

    for diff_a_i in -(h_x as isize)..h_x as isize {
        for diff_a_j in -(w_x as isize)..w_x as isize {
            for diff_b_i in -(h_x as isize)..h_x as isize {
                for diff_b_j in -(w_x as isize)..w_x as isize {
                    let mut set_c = HashSet::new();

                    for &(i, j) in orig_set_a.iter() {
                        set_c.insert((i + diff_a_i, j + diff_a_j));
                    }

                    for &(i, j) in orig_set_b.iter() {
                        set_c.insert((i + diff_b_i, j + diff_b_j));
                    }

                    if set_c == set_x {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
