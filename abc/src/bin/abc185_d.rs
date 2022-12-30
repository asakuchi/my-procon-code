use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_org: [usize; m]
    }

    if m == 0 {
        println!("1");
        return;
    }

    let mut a = Vec::new();

    a.push(0);
    a.append(&mut a_org);
    a.push(n + 1);

    let m = m + 2;

    a.sort();

    let mut spaces = Vec::new();
    let mut min_space = n;

    for i in 1..m {
        let space = a[i] - a[i - 1] - 1;

        if space == 0 {
            continue;
        }

        min_space = min_space.min(space);
        spaces.push(space);
    }

    let mut result = 0;

    for space in spaces {
        if space % min_space == 0 {
            result += space / min_space;
        } else {
            result += 1;
            result += space / min_space;
        }
    }

    println!("{}", result);
}
