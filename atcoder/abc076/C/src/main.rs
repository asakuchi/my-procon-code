use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s_dash: Chars,
        t: Chars,
    }

    let mut candidate = Vec::new();

    for i in 0..s_dash.len() - t.len() + 1 {
        let mut same = true;

        for j in 0..t.len() {
            if s_dash[i + j] != t[j] && s_dash[i + j] != '?' {
                same = false;
            }
        }

        if same {
            let mut s = Vec::new();

            for k in 0..s_dash.len() {
                if k >= i && k < i + t.len() {
                    s.push(t[k - i]);
                } else if s_dash[k] != '?' {
                    s.push(s_dash[k])
                } else {
                    s.push('a');
                }
            }

            candidate.push(s);
        }
    }

    if candidate.len() == 0 {
        println!("UNRESTORABLE");
        return;
    }

    candidate.sort();

    for c in candidate[0].iter() {
        print!("{}", c);
    }

    println!();
}
