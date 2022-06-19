use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut list = vec![0; 4];

    let mut p = 0;

    for &value in &a {
        // マス0に駒を置く
        list[0] += 1;

        let mut next_list = vec![0; 4];

        for i in 0..4 {
            if list[i] > 0 {
                if value + i >= 4 {
                    p += 1;
                } else {
                    next_list[i + value] = list[i];
                }
            }
        }

        list = next_list;
    }

    println!("{}", p);
}
