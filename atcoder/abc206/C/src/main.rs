use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = std::collections::HashMap::new();

    for num in a.iter() {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut result = 0;

    for num in a.iter() {
        let &num_count = map.get(&num).unwrap();

        result += n - num_count;
    }

    result /= 2;

    println!("{}", result);
}
