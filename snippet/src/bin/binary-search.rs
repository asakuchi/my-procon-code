fn main() {
    let mut ok: isize = 1 << 60;
    let mut ng = -1;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || true;

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
