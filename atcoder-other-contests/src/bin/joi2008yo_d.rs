use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        m: usize,
        constellation: [(isize, isize); m],
        n: usize,
        picture: [(isize, isize); n],
    }

    let mut set = HashSet::new();

    for &(p_x, p_y) in picture.iter() {
        set.insert((p_x, p_y));
    }

    let (start_x, start_y) = constellation[0];

    for &(p_x, p_y) in picture.iter() {
        let mut ok = true;

        let (diff_x, diff_y) = (p_x - start_x, p_y - start_y);

        for &(c_x, c_y) in constellation.iter() {
            let point = (c_x + diff_x, c_y + diff_y);

            if !set.contains(&point) {
                ok = false;
                break;
            }
        }

        if !ok {
            continue;
        }

        println!("{} {}", diff_x, diff_y);
        break;
    }
}
