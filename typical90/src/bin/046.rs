use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let a: Vec<_> = a.iter().map(|num| num % 46).collect();
    let b: Vec<_> = b.iter().map(|num| num % 46).collect();
    let c: Vec<_> = c.iter().map(|num| num % 46).collect();

    let mut hash_a = std::collections::HashMap::new();
    let mut hash_b = std::collections::HashMap::new();
    let mut hash_c = std::collections::HashMap::new();

    for &value in a.iter() {
        *hash_a.entry(value).or_insert(0) += 1;
    }

    for &value in b.iter() {
        *hash_b.entry(value).or_insert(0) += 1;
    }

    for &value in c.iter() {
        *hash_c.entry(value).or_insert(0) += 1;
    }

    let mut result = 0 as usize;

    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    let a_count = hash_a.entry(i).or_insert(0);
                    let b_count = hash_b.entry(j).or_insert(0);
                    let c_count = hash_c.entry(k).or_insert(0);

                    result += *a_count * *b_count * *c_count;
                }
            }
        }
    }

    println!("{}", result);
}
