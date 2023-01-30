use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t_x_y_org: [(usize, isize, isize); n],
    }

    let mut t_x_y = vec![(0, 0, 0)];

    t_x_y.append(&mut t_x_y_org);

    for i in 1..=n {
        let diff_t = t_x_y[i].0 - t_x_y[i - 1].0;
        let diff_point = (t_x_y[i].1 - t_x_y[i - 1].1).abs() as usize
            + (t_x_y[i].2 - t_x_y[i - 1].2).abs() as usize;

        if diff_point > diff_t {
            println!("No");
            return;
        }

        // eprintln!("{} {}", diff_t - diff_point, (diff_t - diff_point) % 2);

        if (diff_t - diff_point) % 2 != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
