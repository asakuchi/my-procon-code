use proconio::input;

use ac_library_rs::ModInt998244353;

use ModInt998244353 as mint;

///
/// 期待値
///
fn main() {
    input! {
        n: usize,
        p: usize,
    }

    let attack_1_p = mint::new(1) - mint::new(p) / mint::new(100);
    let attack_2_p = mint::new(p) / mint::new(100);

    // 1回で必ず1以上体力が減るので、必ずn回で倒せる

    // dp
    // ダメージの総和がj以上になる攻撃回数の期待値 E(S)
    let mut dp = vec![mint::new(0); n + 3];

    // ダメージがn以上の状態で必要な攻撃回数は0回
    dp[n] = mint::new(0);

    for i in (0..n).rev() {
        let dp_i1 = dp[i + 1];
        let dp_i2 = dp[i + 2];

        // 状態 S でダメージがjの時、状態S+jに遷移
        // ダメージがjである時の条件付き期待値は、
        // S から S+j にするまでに1回要しているから、
        // E(S+j) + 1 でその確率は attack_j_p
        dp[i] += (dp_i1 + mint::new(1)) * attack_1_p;
        dp[i] += (dp_i2 + mint::new(1)) * attack_2_p;
    }

    println!("{}", dp[0]);
}
