use proconio::fastout;
use proconio::input;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        a: usize,
        mut xh: [(usize, isize); n],
    }

    xh.sort_by_key(|x| x.0);

    let range = d * 2;

    // ダメージの総和
    let mut total: usize = 0;

    // いつまで有効か、ダメージがどれくらいか
    let mut queue = VecDeque::new();

    let mut answer = 0;

    for (x, h) in xh {
        // 有効期限切れの爆弾を探す
        while let Some((effective_position, damage)) = queue.front() {
            if *effective_position < x {
                // 範囲外の爆弾のダメージを減らす
                total -= damage;
                queue.pop_front();
            } else {
                break;
            }
        }

        let h = h - total as isize;

        if h > 0 {
            let num = (h as usize + a - 1) / a;
            answer += num;

            let damage = num * a;
            total += damage;

            queue.push_back((x + range, damage));
        }
    }

    println!("{}", answer);
}
