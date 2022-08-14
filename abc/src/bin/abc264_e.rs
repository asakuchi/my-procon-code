use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        e: usize,
        u_v: [(Usize1, Usize1); e],
        q: usize,
        mut x: [Usize1; q],
    }

    let mut list = vec![Vec::new(); n + m];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let mut uf = UnionFind::new(n + m);

    // 電気が通っているか
    let mut has_light = vec![false; n + m];
    // 都市の数
    let mut town_count = vec![0; n + m];

    // 初期状態 発電所は電気が通っている
    for i in n..n + m {
        has_light[i] = true;
    }

    // 初期状態 都市は都市の数が1
    for i in 0..n {
        town_count[i] = 1;
    }

    // イベントが全て終わった直後の電線
    // 逆から見ていくので初期状態
    let mut init = vec![true; e];

    for &event in &x {
        init[event] = false;
    }

    let mut light_count = 0;

    for i in 0..e {
        if init[i] {
            let (u, v) = u_v[i];

            light_count += update(u, v, &mut town_count, &mut has_light, &mut uf);
        }
    }

    // println!("-------------");
    // println!("light_count:{}", light_count);
    // println!("-------------");

    x.reverse();

    let mut result = Vec::new();

    for &event in &x {
        result.push(light_count);

        let (u, v) = u_v[event];

        light_count += update(u, v, &mut town_count, &mut has_light, &mut uf);
    }

    result.reverse();

    for light in result {
        println!("{}", light);
    }
}

fn update(
    u: usize,
    v: usize,
    town_count: &mut Vec<usize>,
    has_light: &mut Vec<bool>,
    uf: &mut UnionFind<usize>,
) -> usize {
    if uf.equiv(u, v) {
        return 0;
    }

    let mut new_light = 0;

    let town_u = town_count[uf.find(u)];
    let town_v = town_count[uf.find(v)];
    let u_has_light = has_light[uf.find(u)];
    let v_has_light = has_light[uf.find(v)];

    if u_has_light && !v_has_light {
        new_light += town_v;
    }

    if !u_has_light && v_has_light {
        new_light += town_u;
    }

    uf.union(u, v);

    town_count[uf.find(u)] = town_u + town_v;
    has_light[uf.find(u)] = u_has_light || v_has_light;

    // println!(
    //     "u: {} v:{} light_u:{} light_v:{} town_u:{} town_v:{} light_count:{}",
    //     u, v, u_has_light, v_has_light, town_u, town_v, light_count
    // );

    // println!("find:{} {}", uf.find(u), town_u + town_v);
    // println!(
    //     "{:?}",
    //     has_light
    //         .iter()
    //         .enumerate()
    //         .map(|(i, _x)| has_light[uf.find(i)])
    //         .collect::<Vec<_>>()
    // );
    // println!(
    //     "{:?}",
    //     town_count
    //         .iter()
    //         .enumerate()
    //         .map(|(i, _x)| town_count[uf.find(i)])
    //         .collect::<Vec<_>>()
    // );

    new_light
}
