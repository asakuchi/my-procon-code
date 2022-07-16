use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        list: [usize; n],
    }

    let mut set = std::collections::BTreeSet::new();

    let mut result = list.iter().min().unwrap().clone();

    for (i, &num) in list.iter().enumerate() {
        // *map.entry((num, i)).or_insert(0) += 1;
        set.insert((num, i));
    }

    loop {
        let &(max_value, max_i) = set.iter().next_back().unwrap();
        let &(min_value, min_i) = set.iter().next().unwrap();

        // println!("max {} min {}", max_value, min_value);

        if max_value < min_value {
            break;
        }

        result = result.max(min_value);

        if max_value == min_value {
            break;
        }

        if max_value < b {
            break;
        }

        set.remove(&(max_value, max_i));
        set.remove(&(min_value, min_i));

        set.insert((max_value - b, max_i));
        set.insert((min_value + a, min_i));
    }

    println!("{}", result);
}
