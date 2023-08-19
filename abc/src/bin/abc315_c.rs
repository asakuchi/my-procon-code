use proconio::input;

fn main() {
    input! {
        n: usize,
        mut f_s: [(usize, usize); n],
    }

    f_s.sort_by_key(|&(_f, s)| s);

    if f_s.len() == 2 {
        let (f, s) = f_s.pop().unwrap();
        let (f_2, s_2) = f_s.pop().unwrap();

        if f != f_2 {
            println!("{}", s + s_2);
        } else {
            println!("{}", s + s_2 / 2);
        }

        return;
    }

    let mut same_color_second = 0;
    let mut diff_color_max = 0;

    let mut find_same = false;
    let mut find_diff = false;

    let (f_max, s_max) = f_s.pop().unwrap();

    while let Some((f, s)) = f_s.pop() {
        if f_max == f {
            if !find_same {
                same_color_second = s;
            }

            find_same = true;
        } else {
            if !find_diff {
                diff_color_max = s;
            }

            find_diff = true;
        }
    }

    let score_1 = s_max + same_color_second / 2;
    let score_2 = s_max + diff_color_max;

    println!("{}", score_1.max(score_2));
}
