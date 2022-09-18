fn main() {
    let n = input_usize();

    let mut ok: isize = n as isize;
    let mut ng = 0;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        // 質問
        println!("? {} {} {} {}", ng + 1, mid, 1, n);

        // 応答
        let t = input_usize() as isize;

        if t != mid - ng {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let x = ok;

    let mut ok: isize = n as isize;
    let mut ng = 0;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        // 質問
        println!("? {} {} {} {}", 1, n, ng + 1, mid,);

        // 応答
        let t = input_usize() as isize;

        if t != mid - ng {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let y = ok;

    // 回答
    println!("! {} {}", x, y);
}

fn input_usize() -> usize {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    n
}
