use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s_t: [(String, usize); n],
    }

    s_t.sort_by_key(|(_s, t)| *t);

    println!("{}", s_t[s_t.len() - 2].0);
}
