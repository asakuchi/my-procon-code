use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    }

    let targets = vec![p, q, r];

    let result = syaku(&targets, 0, &a);

    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn syaku(targets: &Vec<usize>, step: usize, list: &Vec<usize>) -> bool {
    if step == 3 {
        return true;
    }

    if list.len() == 0 {
        return false;
    }

    // println!("start syaku {:?}", list);

    let mut sum = list[0];

    let target = targets[step];

    // println!("target:{} step:{}", target, step);

    let mut r = 0;

    for l in 0..list.len() {
        while sum < target {
            r += 1;

            if r == list.len() {
                return false;
            }

            sum += list[r];
        }

        // println!("{} {} {}", l, r, sum);

        if sum == target {
            let new_list = &list[r + 1..];
            let new_list = new_list.to_vec();

            if syaku(targets, step + 1, &new_list) {
                return true;
            }
        }

        sum -= list[l];
    }

    false
}
