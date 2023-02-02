use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count_4 = 0;
    let mut count_2 = 0;

    for x in a {
        // 4を約数として持つ要素が半分ほどあれば良い
        // a b*4 c d*4 e

        // 2を約数として持つ要素がnほどあれば代用できる
        // a*2 b*2 c*2 d*4 e

        if x % 4 == 0 {
            count_4 += 1;
        } else if x % 2 == 0 {
            count_2 += 1;
        }
    }

    if count_4 >= n / 2 {
        println!("Yes");
    } else if count_2 >= n - count_4 * 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
