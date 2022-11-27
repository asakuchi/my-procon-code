use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n + q);

    // 代表のボールが入っている箱
    let mut inbox = vec![0; n + q + 1];

    // 箱に入ってるボール（代表値）
    let mut rep_ball = vec![None; n + 1];

    for i in 1..=n {
        inbox[i] = i;
        rep_ball[i] = Some(i);
    }

    let mut k = n;

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                x: usize,
                y: usize,
            }

            if let Some(ball_x) = rep_ball[x] {
                if let Some(ball_y) = rep_ball[y] {
                    uf.union(ball_x, ball_y);

                    rep_ball[y] = None;
                    rep_ball[x] = Some(uf.find(ball_x));

                    inbox[uf.find(ball_x)] = x;
                }

                // y が空なら何もしない
            } else {
                // x が空なら y をそのまま移す
                if let Some(ball_y) = rep_ball[y] {
                    rep_ball[y] = None;
                    rep_ball[x] = Some(ball_y);

                    inbox[ball_y] = x;
                }
            }
        } else if query == 2 {
            input! {
                x: usize,
            }

            k += 1;

            if let Some(ball_x) = rep_ball[x] {
                uf.union(ball_x, k);

                rep_ball[x] = Some(uf.find(ball_x));

                inbox[uf.find(ball_x)] = x;
            } else {
                // x が空なら k を代表値にする
                rep_ball[x] = Some(k);
                inbox[k] = x;
            }
        } else {
            input! {
                x: usize,
            }

            println!("{}", inbox[uf.find(x)]);
        }
    }
}
