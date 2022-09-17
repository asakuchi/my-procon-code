use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let as_text = format!("{:b}", n);
    let keta = as_text.len();

    let mut result = Vec::new();

    rec(n, 0, keta, Vec::new(), &mut result);

    let mut result_2 = Vec::new();

    for values in result {
        let mut digit = 0;

        for i in 0..values.len() {
            if values[i] == 1 {
                digit += 2usize.pow(i as u32);
            }
        }

        result_2.push(digit);
    }

    result_2.sort();

    for value in result_2 {
        println!("{}", value);
    }
}

fn rec(n: usize, index: usize, keta: usize, current: Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if index == keta {
        let new_current = current.clone();
        result.push(new_current);
        return;
    }

    if n & 1 << index > 0 {
        let mut new_current = current.clone();
        new_current.push(1);

        rec(n, index + 1, keta, new_current, result);
    }

    let mut new_current = current.clone();
    new_current.push(0);

    rec(n, index + 1, keta, new_current, result);
}
