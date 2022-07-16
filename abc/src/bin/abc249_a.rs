use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }

    let mut taka = 0;
    let mut aoki = 0;

    let mut walk_t = a;
    let mut walk_a = d;

    let mut rest_t = 0;
    let mut rest_a = 0;

    for _i in 0..x {
        if walk_t > 0 {
            taka += b;
            walk_t -= 1;

            if walk_t == 0 {
                rest_t += c;
            }
        } else if rest_t > 0 {
            rest_t -= 1;

            if rest_t == 0 {
                walk_t = a;
            }
        }

        if walk_a > 0 {
            aoki += e;
            walk_a -= 1;

            if walk_a == 0 {
                rest_a += f;
            }
        } else if rest_a > 0 {
            rest_a -= 1;

            if rest_a == 0 {
                walk_a = d;
            }
        }
    }

    if taka > aoki {
        println!("Takahashi");
    } else if taka < aoki {
        println!("Aoki");
    } else {
        println!("Draw ");
    }
}
