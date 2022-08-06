use proconio::input;

fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        a: [isize; n],
    }

    let mut x_total = 0;
    let mut x_total_list = vec![0; n];

    for i in 0..n {
        x_total += l - a[i];
        x_total_list[i] = x_total;
    }

    let mut y_total = 0;
    let mut y_total_list = vec![0; n];

    for i in (0..n).rev() {
        y_total += r - a[i];
        y_total_list[i] = y_total;
    }

    let mut x_best_index = vec![Option::None; n];
    let mut y_best_index = vec![Option::None; n];

    let mut current = 0;
    let mut index = Option::None;

    for i in 0..n {
        if x_total_list[i] < current {
            current = x_total_list[i];
            index = Some(i);
        }

        x_best_index[i] = index;
    }

    let mut current = 0;
    let mut index = Option::None;

    for i in (0..n).rev() {
        if y_total_list[i] < current {
            current = y_total_list[i];
            index = Some(i);
        }

        y_best_index[i] = index;
    }

    // println!("x_total_list {:?}", x_total_list);
    // println!("y_total_list {:?}", y_total_list);

    // println!("x_best_index {:?}", x_best_index);
    // println!("y_best_index {:?}", y_best_index);

    let mut result = 0;

    // x: None y:0
    // 全てY
    result = result.min(y_total_list[0]);

    // x: n-1 y:None
    // 全てX
    result = result.min(x_total_list[n - 1]);

    for i in 0..n - 1 {
        let x_i = x_best_index[i];
        let y_i = y_best_index[i + 1];

        let mut score = 0;

        if let Some(index) = x_i {
            score += x_total_list[index];
        }

        if let Some(index) = y_i {
            score += y_total_list[index];
        }

        // println!("i:{} score:{}", i, score);

        result = result.min(score);
    }

    let a_sum = a.iter().sum::<isize>();

    println!("{}", a_sum + result);
}
