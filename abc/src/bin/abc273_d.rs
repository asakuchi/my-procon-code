use std::collections::BTreeSet;

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: isize,
        w: isize,
        start: (isize, isize),
        n: usize,
        r_c: [(isize, isize); n],
        q: usize,
        d_l: [(char, isize); q],
    }

    let mut r_walls = BTreeSet::new();
    let mut c_walls = BTreeSet::new();

    for i in 0..n {
        let (r, c) = r_c[i];

        r_walls.insert((r, c));
        c_walls.insert((c, r));
    }

    let mut current = start;

    for (d, l) in d_l {
        // println!("current {:?}", current);

        if d == 'L' {
            if let Some(&wall) = r_walls.range(..(current.0, current.1)).last() {
                // println!("L wall {:?}", wall);

                if wall.0 == current.0 && wall.1 >= current.1 - l {
                    current.1 = wall.1 + 1;
                } else {
                    current.1 = current.1 - l;
                }
            } else {
                current.1 = current.1 - l;
            }
        } else if d == 'R' {
            if let Some(&wall) = r_walls.range((current.0, current.1 + 1)..).next() {
                // println!("R wall {:?}", wall);

                if wall.0 == current.0 && wall.1 <= current.1 + l {
                    current.1 = wall.1 - 1;
                } else {
                    current.1 = current.1 + l;
                }
            } else {
                current.1 = current.1 + l;
            }
        } else if d == 'U' {
            if let Some(&wall) = c_walls.range(..(current.1, current.0)).last() {
                // println!("U wall {:?}", wall);

                // wall 逆
                let wall = (wall.1, wall.0);

                if wall.1 == current.1 && wall.0 >= current.0 - l {
                    current.0 = wall.0 + 1;
                } else {
                    current.0 = current.0 - l;
                }
            } else {
                current.0 = current.0 - l;
            }
        } else {
            if let Some(&wall) = c_walls.range((current.1, current.0 + 1)..).next() {
                // println!("D wall {:?}", wall);

                // wall 逆
                let wall = (wall.1, wall.0);

                if wall.1 == current.1 && wall.0 <= current.0 + l {
                    current.0 = wall.0 - 1;
                } else {
                    current.0 = current.0 + l;
                }
            } else {
                current.0 = current.0 + l;
            }
        }

        if current.0 < 1 {
            current.0 = 1;
        }

        if current.0 > h {
            current.0 = h;
        }

        if current.1 < 1 {
            current.1 = 1;
        }

        if current.1 > w {
            current.1 = w;
        }

        println!("{} {}", current.0, current.1);
    }
}
