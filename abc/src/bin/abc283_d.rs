use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let kinsi = vec![false; 26];

    let result = rec(&s, &kinsi, 0);

    if result.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn rec(s: &Vec<char>, kinsi: &Vec<bool>, index: usize) -> (bool, usize) {
    let mut copy = kinsi.clone();

    let mut i = index;

    while i < s.len() {
        let c = s[i];

        if c == '(' {
            let res = rec(s, &copy, i + 1);

            if !res.0 {
                return (false, 0);
            }

            i = res.1;
        } else if c == ')' {
            return (true, i);
        } else {
            let c_index = (c as u8 - 'a' as u8) as usize;

            if copy[c_index] {
                return (false, 0);
            }

            copy[c_index] = true;
        }

        i += 1;
    }

    (true, i)
}
