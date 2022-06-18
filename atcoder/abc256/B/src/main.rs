use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut list = vec![0; 4];

    let mut p = 0;

    for &value in &a {
        list[0] += 1;

        for i in (0..4).rev() {
            if value + i >= 4 {
                if list[i] > 0 {
                    p += 1;
                }

                list[i] = 0;
            } else {
                list[i + value] = list[i];
                list[i] = 0;
            }
        }
    }

    println!("{}", p);
}
