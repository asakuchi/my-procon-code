use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut set = std::collections::HashSet::new();

    for i in 0..=12 {
        let std_num_str = format!("{}{}", k, "0".repeat(i));

        if let Ok(std_num) = std_num_str.parse() {
            if 1usize <= std_num && std_num <= n && f(std_num) == k {
                set.insert(std_num);
            }
        }

        let rev_num_str = format!("{}{}", reverse_num(k), "0".repeat(i));

        if let Ok(rev_num) = rev_num_str.parse() {
            if 1usize <= rev_num && rev_num <= n && f(rev_num) == k {
                set.insert(rev_num);
            }
        }
    }

    println!("{}", set.len());
}

fn reverse_num(x: usize) -> usize {
    let str_x = x.to_string();

    let rev_str: String = str_x.chars().rev().collect();

    if let Ok(rev_num) = rev_str.parse() {
        rev_num
    } else {
        0
    }
}

fn f(x: usize) -> usize {
    let mut min_value = x;

    let rev_x = reverse_num(x);

    min_value = min_value.min(rev_x);

    min_value = min_value.min(reverse_num(rev_x));

    min_value
}
