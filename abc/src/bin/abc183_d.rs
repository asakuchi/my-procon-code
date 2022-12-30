use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        s_t_p: [(usize, usize, isize); n],
    }

    let mut imos = vec![0isize; 200_100];

    for &(s, t, p) in &s_t_p {
        imos[s] += p;
        imos[t] -= p;
    }

    for i in 0..=200_000 {
        if i > 0 {
            imos[i] += imos[i - 1];
        }

        if imos[i] > w as isize {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
