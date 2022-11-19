use proconio::input;

fn main() {
    input! {
        h: usize,
        m: usize,
    };

    for i in m..=59 {
        if let Some((_next_h, _next_m)) = miss(h, i) {
            println!("{} {}", h, i);
            return;
        }
    }

    for j in h + 1..=23 {
        for i in 0..=59 {
            if let Some((_next_h, _next_m)) = miss(j, i) {
                println!("{} {}", j, i);
                return;
            }
        }
    }

    for j in 0..=23 {
        for i in 0..=59 {
            if let Some((_next_h, _next_m)) = miss(j, i) {
                println!("{} {}", j, i);
                return;
            }
        }
    }
}

fn miss(h: usize, m: usize) -> Option<(usize, usize)> {
    let h_text = format!("{:02}", h);
    let m_text = format!("{:02}", m);

    let new_h = format!(
        "{}{}",
        h_text.chars().nth(0).unwrap(),
        m_text.chars().nth(0).unwrap()
    );
    let new_m = format!(
        "{}{}",
        h_text.chars().nth(1).unwrap(),
        m_text.chars().nth(1).unwrap()
    );

    let new_h: usize = new_h.parse().unwrap();
    let new_m: usize = new_m.parse().unwrap();

    if new_h <= 23 && new_m <= 59 {
        Some((new_h, new_m))
    } else {
        None
    }
}
