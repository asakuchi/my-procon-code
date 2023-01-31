use proconio::{input, marker::Chars};

fn main() {
    input! {
        abcd: Chars,
    }

    let list: Vec<isize> = abcd
        .iter()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    for op1 in 0..=1 {
        for op2 in 0..=1 {
            for op3 in 0..=1 {
                let mut sum = list[0];

                sum = exec(op1, sum, list[1]);
                sum = exec(op2, sum, list[2]);
                sum = exec(op3, sum, list[3]);

                if sum == 7 {
                    println!(
                        "{}{}{}{}{}{}{}=7",
                        list[0],
                        display(op1),
                        list[1],
                        display(op2),
                        list[2],
                        display(op3),
                        list[3],
                    );
                    return;
                }
            }
        }
    }
}

fn exec(ope: usize, lhs: isize, rhs: isize) -> isize {
    if ope == 0 {
        lhs + rhs
    } else {
        lhs - rhs
    }
}

fn display(ope: usize) -> String {
    if ope == 0 {
        "+".into()
    } else {
        "-".into()
    }
}
