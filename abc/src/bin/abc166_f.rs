use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        s: [String; n],
    }

    let mut result = Vec::new();

    if rec(n, &s, a, b, c, 0, &mut result) {
        println!("Yes");

        result.reverse();

        for select in result {
            println!("{}", select);
        }
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    s: &Vec<String>,
    a: usize,
    b: usize,
    c: usize,
    index: usize,
    result: &mut Vec<char>,
) -> bool {
    if index == n {
        return true;
    }

    match s[index].as_str() {
        "AB" => {
            if a > 0 {
                if rec(n, s, a - 1, b + 1, c, index + 1, result) {
                    result.push('B');

                    return true;
                }
            }

            if b > 0 {
                if rec(n, s, a + 1, b - 1, c, index + 1, result) {
                    result.push('A');

                    return true;
                }
            }
        }
        "BC" => {
            if b > 0 {
                if rec(n, s, a, b - 1, c + 1, index + 1, result) {
                    result.push('C');

                    return true;
                }
            }

            if c > 0 {
                if rec(n, s, a, b + 1, c - 1, index + 1, result) {
                    result.push('B');

                    return true;
                }
            }
        }
        _ => {
            if c > 0 {
                if rec(n, s, a + 1, b, c - 1, index + 1, result) {
                    result.push('A');

                    return true;
                }
            }

            if a > 0 {
                if rec(n, s, a - 1, b, c + 1, index + 1, result) {
                    result.push('C');

                    return true;
                }
            }
        }
    }

    false
}
