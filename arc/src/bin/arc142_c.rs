use std::io;

fn main() {
    let n = input_n();

    let mut min = 1_000_000_000;

    let mut set_3 = std::collections::HashSet::new();

    for v in 3..=n {
        let d_1_v = question(1, v);
        let d_2_v = question(2, v);

        let d_i = d_1_v + d_2_v;

        min = min.min(d_i);

        if d_i == 3 {
            set_3.insert(v);
        }
    }

    if min != 3 {
        println!("! {}", min);
        return;
    }

    if set_3.len() != 2 {
        println!("! {}", 1);
        return;
    }

    let mut iter = set_3.iter();

    let &u = iter.next().unwrap();
    let &v = iter.next().unwrap();

    let d_u_v = question(u, v);

    if d_u_v == 1 {
        println!("! {}", 3);
    } else {
        println!("! {}", 1);
    }
}

fn input_n() -> usize {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();
    // ------------------------------------

    n
}

fn question(u: usize, v: usize) -> usize {
    println!("? {} {}", u, v);

    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let d_uv: usize = buf.parse().unwrap();
    // ------------------------------------

    d_uv
}
