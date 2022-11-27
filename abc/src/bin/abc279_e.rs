use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    }

    let mut top = vec![0; m + 1];
    let mut one_position = 0;

    top[0] = 0;

    for i in 0..m {
        let v = a[i];

        if v == one_position {
            one_position += 1;
        } else if v + 1 == one_position {
            one_position -= 1;
        }

        top[i + 1] = one_position;
    }

    let mut bottom: Vec<_> = (0..n).collect();

    let mut result = Vec::new();

    for i in (1..m + 1).rev() {
        let target = top[i - 1];

        result.push(bottom[target]);

        let v = a[i - 1];

        bottom.swap(v, v + 1);

        // println!("i {}", i);
        // println!("{:?}", bottom);
        // println!("{}", top[i - 1]);
        // println!();
    }

    // println!("{:?}", result);

    result.reverse();

    for value in result {
        println!("{}", value + 1);
    }
}
