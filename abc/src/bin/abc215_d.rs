use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    if m == 1 {
        println!("1");
        println!("1");
        return;
    }

    let mut ban = std::collections::HashSet::new();

    for &num in a.iter() {
        ban.insert(num);

        let mut current = num;

        {
            let mut i = 2;

            while i * i <= num {
                while current % i == 0 {
                    // list.push(i);
                    ban.insert(i);
                    current /= i;
                }

                if current == 1 {
                    break;
                }

                i += 1;
            }

            if current != 1 {
                ban.insert(current);
            }
        }
    }

    ban.remove(&1);

    for &i in ban.clone().iter() {
        for j in (i * 2..=m).step_by(i) {
            ban.insert(j);
        }
    }

    let mut result = Vec::new();

    for num in 1..=m {
        if !ban.contains(&num) {
            result.push(num);
        }
    }

    result.sort();

    println!("{}", result.len());

    for &num in result.iter() {
        println!("{}", num);
    }
}
