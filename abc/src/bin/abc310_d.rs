// 解説AC
use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut ng = vec![HashSet::new(); n];

    for &(a, b) in &a_b {
        ng[a].insert(b);
        ng[b].insert(a);
    }

    let mut teams = Vec::new();

    let mut current_team: Vec<_> = vec![Vec::new(); t];

    rec(n, &ng, t, 0, &mut current_team, &mut teams);

    println!("{}", teams.len());
}

fn rec(
    n: usize,
    ng: &Vec<HashSet<usize>>,
    t: usize,
    player: usize,
    current_team: &Vec<Vec<usize>>,
    teams: &mut Vec<Vec<Vec<usize>>>,
) {
    if player == n {
        for i in 0..t {
            if current_team[i].len() == 0 {
                // NG
                return;
            }
        }

        teams.push(current_team.clone());

        return;
    }

    'search_team: for i in 0..t {
        for &member in current_team[i].iter() {
            if ng[player].contains(&member) {
                continue 'search_team;
            }
        }

        let mut current_team_copy = current_team.clone();
        current_team_copy[i].push(player);

        rec(n, ng, t, player + 1, &current_team_copy, teams);

        if current_team[i].len() == 0 {
            // 空のチームがあるならそれを飛ばして次のチームに入れる場合を考えなくて良い
            break;
        }
    }
}
