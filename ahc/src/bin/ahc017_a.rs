use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::BinaryHeap;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    input! {
        n: usize,
        m: usize,
        d: usize,
        k: usize,
        mut u_v_w: [(Usize1, Usize1, i32); m],
        x_y: [(i32, i32); n],
    }

    // -------------------------------
    //
    // 初期化
    //
    // -------------------------------
    // ある工事日にどの道路を工事するか
    let end = start.elapsed();
    eprintln!("initializing... {}", end.as_millis());

    let plan_day = initialize(n, m, d, &mut u_v_w);

    let end = start.elapsed();
    eprintln!("initialized {}", end.as_millis());

    // -------------------------------

    // 山登り

    // tools で定義されたinput
    let input = Input {
        D: d,
        K: k,
        ps: x_y,
        es: u_v_w,
    };

    let mut rng = rand::thread_rng();

    let output = to_output(plan_day, m, d);

    let end = start.elapsed();
    eprintln!("computing score... {}", end.as_millis());

    let (score, _err, _) = compute_score(&input, &output);

    let end = start.elapsed();
    eprintln!("computed {}", end.as_millis());

    eprintln!("score:{}", score);

    loop {
        let end = start.elapsed();
        if end.as_millis() >= 1_000 {
            // eprintln!("timeup");

            break;
        }
    }

    // -------------------------------
    //
    // 出力
    //
    // -------------------------------

    // let mut result = vec![0; m];

    // for day in 0..d {
    //     for &load in &plan_day[day] {
    //         result[load] = day + 1;
    //     }
    // }

    let text = &output[..m].iter().format(" ");

    println!("{}", text);
}

fn initialize(
    n: usize,
    m: usize,
    d: usize,
    u_v_w: &mut Vec<(usize, usize, i32)>,
) -> Vec<Vec<usize>> {
    // 最小全域木
    let mut min_spanning_tree = Vec::new();
    // その他
    let mut others = Vec::new();

    // 重みの昇順にソート
    u_v_w.sort_by_key(|x| x.2);

    let mut set = UnionFind::new(n);

    for i in 0..m {
        let (u, v, _w) = u_v_w[i];

        if !set.equiv(u, v) {
            min_spanning_tree.push(i);
            set.union(u, v);
        } else {
            others.push(i);
        }
    }

    // eprintln!("mst {:?}", min_spanning_tree);
    eprintln!("mst {}", min_spanning_tree.len());

    // 各日に工事する道路
    let mut plan_day = vec![Vec::new(); d];

    let mut days = (0..d).cycle();

    // 最小全域木を均等に割り振る
    for &load in min_spanning_tree.iter() {
        let day = days.next().unwrap();

        plan_day[day].push(load);
    }

    // その他も均等に割り振る
    for &load in others.iter() {
        let day = days.next().unwrap();

        plan_day[day].push(load);
    }

    plan_day
}

fn to_output(plan_day: Vec<Vec<usize>>, m: usize, d: usize) -> Output {
    let mut result = vec![0; m];

    for day in 0..d {
        for &load in &plan_day[day] {
            result[load] = day + 1;
        }
    }

    result
}

///
/// tools から拝借
///
pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

pub const INF: i32 = 1_000_000_000;

pub type Output = Vec<usize>;

#[derive(Clone, Debug)]
pub struct Input {
    pub D: usize,
    pub K: usize,
    pub ps: Vec<(i32, i32)>,
    pub es: Vec<(usize, usize, i32)>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} {}",
            self.ps.len(),
            self.es.len(),
            self.D,
            self.K
        )?;
        for &(u, v, w) in &self.es {
            writeln!(f, "{} {} {}", u + 1, v + 1, w)?;
        }
        for p in &self.ps {
            writeln!(f, "{} {}", p.0, p.1)?;
        }
        Ok(())
    }
}

fn get_graph(input: &Input, out: &Output, day: usize) -> Vec<Vec<(usize, i32)>> {
    let mut g = vec![vec![]; input.ps.len()];
    for e in 0..input.es.len() {
        if out[e] != day {
            let (u, v, w) = input.es[e];
            g[u].push((v, w));
            g[v].push((u, w));
        }
    }
    g
}

fn compute_dist(g: &Vec<Vec<(usize, i32)>>, s: usize) -> Vec<i32> {
    let mut dist = vec![INF; g.len()];
    let mut que = BinaryHeap::new();
    que.push((0, s));
    dist[s] = 0;
    while let Some((d, u)) = que.pop() {
        let d = -d;
        if dist[u] != d {
            continue;
        }
        for &(v, w) in &g[u] {
            let d2 = d + w;
            if dist[v].setmin(d2) {
                que.push((-d2, v));
            }
        }
    }
    dist
}

fn compute_dist_matrix(input: &Input, out: &Output, day: usize) -> Vec<Vec<i32>> {
    let g = get_graph(input, out, day);
    let mut dist = vec![];
    for s in 0..input.ps.len() {
        dist.push(compute_dist(&g, s));
    }
    dist
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String, Vec<f64>) {
    let mut count = vec![0; input.D + 1];
    for i in 0..input.es.len() {
        count[out[i]] += 1;
    }
    for i in 1..=input.D {
        if count[i] > input.K {
            return (
                0,
                format!(
                    "The number of edges to be repaired on day {} has exceeded the limit. ({} > {})",
                    i, count[i], input.K
                ),
                vec![],
            );
        }
    }
    let mut num = 0;
    let dist0 = compute_dist_matrix(input, out, !0);
    let mut fs = vec![];
    for day in 1..=input.D {
        let dist = compute_dist_matrix(input, out, day);
        let mut tmp = 0;
        for i in 0..input.ps.len() {
            for j in i + 1..input.ps.len() {
                tmp += (dist[i][j] - dist0[i][j]) as i64;
            }
        }
        num += tmp;
        fs.push(tmp as f64 / (input.ps.len() * (input.ps.len() - 1) / 2) as f64);
    }
    let den = input.D * input.ps.len() * (input.ps.len() - 1) / 2;
    let avg = num as f64 / den as f64 * 1000.0;
    (avg.round() as i64, String::new(), fs)
}

fn dist2((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}
