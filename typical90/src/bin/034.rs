use std::collections::HashMap;
use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut result = 0;
    let mut list = VecDeque::new();

    let mut map = HashMap::new();
    let mut variables = 0;

    for (right, &right_value) in a.iter().enumerate() {
        list.push_back((right, right_value));

        let count = map.entry(right_value).or_insert(0);

        if *count == 0 {
            variables += 1;
        }

        *count += 1;

        while variables > k {
            if let Some((left, left_value)) = list.pop_front() {
                let count = map.entry(left_value).or_insert(0);

                *count -= 1;

                if *count == 0 {
                    variables -= 1;
                }
            } else {
                break;
            }
        }

        // println!("{:?}", list);

        result = result.max(list.len());
    }

    println!("{}", result);
}
