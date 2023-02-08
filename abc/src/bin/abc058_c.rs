use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut char_count = vec![50; 26];

    for text in s {
        let mut si_count = vec![0; 26];

        for c in text {
            si_count[c as usize - 'a' as usize] += 1;
        }

        for c in 'a' as u8..='z' as u8 {
            let i = c as usize - 'a' as usize;
            char_count[i] = char_count[i].min(si_count[i]);
        }
    }

    // println!("{:?}", char_count);

    let mut result = String::new();

    for c in 'a' as u8..='z' as u8 {
        let i = c as usize - 'a' as usize;

        result += (c as char).to_string().repeat(char_count[i]).as_str();
    }

    println!("{}", result);
}
