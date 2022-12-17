use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let mut color_list = vec![0; n];

    let mut tree_list = Vec::new();

    for i in 0..n {
        if color_list[i] != 0 {
            continue;
        }

        let check = dfs(n, &list, i, 1, &mut color_list);

        if check {
            tree_list.push(i);
        } else {
            println!("0");
            return;
        }
    }

    // 二部グラフな連結成分だけ色を数える
    let mut color_count = Vec::new();
    let mut visited = vec![false; n];

    for &vertex in &tree_list {
        let mut color_1_count = 0;
        let mut color_2_count = 0;

        let mut v_count = 0;

        visited[vertex] = true;

        count_color(
            n,
            &list,
            vertex,
            &mut color_1_count,
            &mut color_2_count,
            &color_list,
            &mut visited,
            &mut v_count,
        );

        color_count.push((color_1_count, color_2_count, v_count));
    }

    // 二部グラフな連結成分だけチェックする
    let mut visited = vec![false; n];

    let mut result = 0;

    let mut other_result = 0;

    for i in 0..tree_list.len() {
        let vertex = tree_list[i];

        visited[vertex] = true;

        let (total_color_1_count, total_color_2_count, v_count) = color_count[i];

        let score = dfs_2(
            n,
            &list,
            vertex,
            &color_list,
            total_color_1_count,
            total_color_2_count,
            &mut visited,
        ) / 2;

        result += score;

        other_result += v_count * (n - v_count);
    }

    println!("{}", result + other_result / 2);
}

fn dfs(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    color: usize,
    color_list: &mut Vec<usize>,
) -> bool {
    color_list[current] = color;

    let opposit_color = if color == 1 { 2 } else { 1 };

    for &next in list[current].iter() {
        if color_list[next] == color {
            // NG
            return false;
        }

        if color_list[next] != 0 {
            continue;
        }

        let check = dfs(n, list, next, opposit_color, color_list);

        if !check {
            return false;
        }
    }

    true
}

fn count_color(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    color_1_count: &mut usize,
    color_2_count: &mut usize,
    color_list: &Vec<usize>,
    visited: &mut Vec<bool>,
    v_count: &mut usize,
) {
    if color_list[current] == 1 {
        *color_1_count += 1;
    } else {
        *color_2_count += 1;
    }

    *v_count += 1;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        count_color(
            n,
            list,
            next,
            color_1_count,
            color_2_count,
            color_list,
            visited,
            v_count,
        );
    }
}

fn dfs_2(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    color_list: &Vec<usize>,
    total_color_1_count: usize,
    total_color_2_count: usize,
    visited: &mut Vec<bool>,
) -> usize {
    // 直接の子供の数
    let mut color_1_count = 0;
    let mut color_2_count = 0;

    let mut result = 0;

    for &next in list[current].iter() {
        if color_list[next] == 1 {
            color_1_count += 1;
        } else {
            color_2_count += 1;
        }

        if visited[next] {
            continue;
        }

        visited[next] = true;

        let score = dfs_2(
            n,
            list,
            next,
            color_list,
            total_color_1_count,
            total_color_2_count,
            visited,
        );

        result += score;
    }

    if color_list[current] == 1 {
        result += total_color_2_count - color_2_count;
    } else {
        result += total_color_1_count - color_1_count;
    }

    result
}
