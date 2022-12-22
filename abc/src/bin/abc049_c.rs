use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let list = vec![
        String::from("dream"),
        String::from("dreamer"),
        String::from("erase"),
        String::from("eraser"),
    ];

    let result = rec(&s, &list);

    if result {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn rec(s: &str, list: &Vec<String>) -> bool {
    if s.len() == 0 {
        return true;
    }

    for text in list {
        if s.starts_with(text) {
            if rec(&s[text.len()..], list) {
                return true;
            }
        }
    }

    false
}
