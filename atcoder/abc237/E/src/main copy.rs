///
/// ダイクストラ
/// RE
///
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        uv_org: [(usize, usize); m],
    }

    let min_enjoy = -2 * 10i128.pow(10);

    let mut dist = vec![vec![min_enjoy; n]; n];

    let uv = uv_org
        .into_iter()
        .map(|(x, y)| (x - 1, y - 1))
        .collect::<Vec<_>>();

    for (u, v) in &uv {
        dist[*u][*v] = if h[*u] > h[*v] {
            (h[*u] - h[*v]) as i128
        } else {
            -2 * (h[*v] - h[*u]) as i128
        };

        dist[*v][*u] = if h[*v] > h[*u] {
            (h[*v] - h[*u]) as i128
        } else {
            -2 * (h[*u] - h[*v]) as i128
        };
    }

    let mut edge = vec![Vec::new(); n];

    for (u, v) in &uv {
        edge[*u].push(v);
        edge[*v].push(u);
    }

    // startからの楽しさ
    let mut d = vec![min_enjoy; n];
    let mut check = vec![false; n];

    d[0] = 0;

    // println!("edge:{:?}", edge);
    // println!("dist:{:?}", dist);

    for _ in 0..n {
        // println!("{:?}", d);

        let mut now = 0;
        let mut nowd = min_enjoy;

        for j in 0..n {
            if nowd < d[j] && !check[j] {
                nowd = d[j];
                now = j;
            }
        }

        // println!("now nowd:{} {}", now, nowd);

        if nowd == min_enjoy {
            break;
        }

        check[now] = true;

        for &next in &edge[now] {
            let nextd = d[now] + dist[now][*next];

            // println!(
            //     "更新 d[now] dist[now][j] d[*next]:{} {} {}",
            //     d[now], dist[now][*next], d[*next]
            // );

            if nextd > d[*next] {
                d[*next] = nextd;
            }
        }
    }

    let mut max = min_enjoy;

    // println!("{:?}", d);

    for value in d {
        max = std::cmp::max(max, value);
    }

    println!("{}", max);
}
